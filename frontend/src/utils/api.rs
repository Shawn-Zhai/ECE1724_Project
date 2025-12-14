use anyhow::Result;
use serde_json::json;

use super::app::{App, InputState, Mode};
use super::model::{Account, Category, CreateSplit, CreateTransaction, DirectionKind, Transaction};

pub async fn submit_transaction(app: &mut App) -> Result<()> {
    let amount: f64 = app
        .input
        .amount
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid amount"))?;

    if amount < 0.0 {
        app.status = "Amount must be non-negative".into();
        return Ok(());
    }

    let description = if app.input.description.is_empty() {
        None
    } else {
        Some(app.input.description.clone())
    };

    let client = reqwest::Client::new();
    let res = if app.mode == Mode::Transfer {
        let from = app
            .accounts
            .get(app.input.account_idx)
            .ok_or_else(|| anyhow::anyhow!("No source account available"))?;
        let to = app
            .accounts
            .get(app.input.to_account_idx)
            .ok_or_else(|| anyhow::anyhow!("No destination account available"))?;
        if from.id == to.id {
            app.status = "Source and destination must differ".into();
            return Ok(());
        }
        let payload = CreateTransaction {
            account_id: from.id.clone(),
            to_account_id: Some(to.id.clone()),
            amount,
            direction: DirectionKind::Transfer,
            description,
            occurred_at: None,
            splits: None,
        };
        if let Some(edit_id) = app.editing_txn_id.clone() {
            client
                .put(format!("{}/transactions/{}", app.backend_url, edit_id))
                .json(&payload)
                .send()
                .await?
        } else {
            client
                .post(format!("{}/transactions", app.backend_url))
                .json(&payload)
                .send()
                .await?
        }
    } else {
        let account = app
            .accounts
            .get(app.input.account_idx)
            .ok_or_else(|| anyhow::anyhow!("No account available"))?;
        let category = app
            .categories
            .get(app.input.category_idx)
            .ok_or_else(|| anyhow::anyhow!("No category available"))?;

        let payload = CreateTransaction {
            account_id: account.id.clone(),
            to_account_id: None,
            amount,
            direction: app.input.direction.clone(),
            description,
            occurred_at: None,
            splits: Some(vec![CreateSplit {
                category_id: category.id.clone(),
                amount,
            }]),
        };
        if let Some(edit_id) = app.editing_txn_id.clone() {
            client
                .put(format!("{}/transactions/{}", app.backend_url, edit_id))
                .json(&payload)
                .send()
                .await?
        } else {
            client
                .post(format!("{}/transactions", app.backend_url))
                .json(&payload)
                .send()
                .await?
        }
    };

    if res.status().is_success() {
        app.status = if app.editing_txn_id.is_some() {
            "Transaction updated".into()
        } else {
            "Transaction saved".into()
        };
        app.editing_txn_id = None;
        app.input = InputState {
            direction: DirectionKind::Expense,
            ..Default::default()
        };
        app.mode = Mode::Normal;
        refresh(app).await?;
    } else {
        let text = res.text().await.unwrap_or_else(|_| "unknown error".into());
        app.status = format!("Failed to create: {text}");
    }
    Ok(())
}

pub async fn refresh(app: &mut App) -> Result<()> {
    let client = reqwest::Client::new();
    let accounts: Vec<Account> = client
        .get(format!("{}/accounts", app.backend_url))
        .send()
        .await?
        .json()
        .await?;
    let categories: Vec<Category> = client
        .get(format!("{}/categories", app.backend_url))
        .send()
        .await?
        .json()
        .await?;
    let transactions: Vec<Transaction> = client
        .get(format!("{}/transactions", app.backend_url))
        .send()
        .await?
        .json()
        .await?;

    app.accounts = accounts;
    app.categories = categories;
    app.transactions = transactions;
    if !app.transactions.is_empty() {
        app.selected_txn_idx = app.selected_txn_idx.min(app.transactions.len().saturating_sub(1));
    } else {
        app.selected_txn_idx = 0;
    }
    app.status = format!(
        "{} accounts | {} categories | {} transactions",
        app.accounts.len(),
        app.categories.len(),
        app.transactions.len()
    );
    Ok(())
}

pub async fn create_account(app: &mut App, name: &str, kind: &str) -> Result<()> {
    if name.trim().is_empty() {
        app.status = "Account name cannot be empty".into();
        return Ok(());
    }

    let client = reqwest::Client::new();
    let payload = json!({
        "name": name,
        "kind": kind,
    });
    let res = client
        .post(format!("{}/accounts", app.backend_url))
        .json(&payload)
        .send()
        .await?;
    if res.status().is_success() {
        refresh(app).await?;
        app.status = format!("Account \"{}\" created", name);
    } else {
        let text = res.text().await.unwrap_or_else(|_| "unknown error".into());
        app.status = format!("Failed to create account: {text}");
    }
    Ok(())
}

pub async fn delete_transaction(app: &mut App, txn_id: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .delete(format!("{}/transactions/{}", app.backend_url, txn_id))
        .send()
        .await?;
    if res.status().is_success() {
        refresh(app).await?;
        app.status = "Transaction deleted".into();
    } else {
        let text = res.text().await.unwrap_or_else(|_| "unknown error".into());
        app.status = format!("Failed to delete transaction: {text}");
    }
    Ok(())
}

pub async fn delete_account(app: &mut App, account_id: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .delete(format!("{}/accounts/{}", app.backend_url, account_id))
        .send()
        .await?;
    if res.status().is_success() {
        refresh(app).await?;
        app.status = "Account deleted".into();
    } else {
        let text = res.text().await.unwrap_or_else(|_| "unknown error".into());
        app.status = format!("Failed to delete account: {text}");
    }
    Ok(())
}
