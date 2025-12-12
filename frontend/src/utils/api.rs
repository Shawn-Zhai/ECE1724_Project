use anyhow::Result;

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
        amount,
        direction: app.input.direction.clone(),
        description: if app.input.description.is_empty() {
            None
        } else {
            Some(app.input.description.clone())
        },
        occurred_at: None,
        splits: Some(vec![CreateSplit {
            category_id: category.id.clone(),
            amount,
        }]),
    };

    let url = format!("{}/transactions", app.backend_url);
    let client = reqwest::Client::new();
    let res = client.post(url).json(&payload).send().await?;
    if res.status().is_success() {
        app.status = "Transaction added".into();
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
    app.status = format!(
        "{} accounts | {} categories | {} transactions",
        app.accounts.len(),
        app.categories.len(),
        app.transactions.len()
    );
    Ok(())
}
