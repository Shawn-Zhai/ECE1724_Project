use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub balance: f64,
    pub created_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DirectionKind {
    Income,
    Expense,
    Transfer,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct TransactionSplit {
    pub transaction_id: String,
    pub category_id: String,
    pub amount: f64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub account_id: String,
    pub amount: f64,
    pub direction: DirectionKind,
    pub description: Option<String>,
    pub occurred_at: String,
    pub splits: Vec<TransactionSplit>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct CreateTransaction {
    pub account_id: String,
    pub amount: f64,
    pub direction: DirectionKind,
    pub description: Option<String>,
    pub occurred_at: Option<String>,
    pub splits: Option<Vec<CreateSplit>>,
}

#[derive(Serialize)]
pub struct CreateSplit {
    pub category_id: String,
    pub amount: f64,
}
