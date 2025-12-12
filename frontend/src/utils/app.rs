use super::model::{Account, Category, DirectionKind, Transaction};

#[derive(Clone)]
pub struct InputState {
    pub account_idx: usize,
    pub category_idx: usize,
    pub active_field: ActiveField,
    pub direction: DirectionKind,
    pub amount: String,
    pub description: String,
    pub to_account_idx: usize,
    pub new_account_name: String,
    pub new_account_kind_idx: usize,
}

impl Default for ActiveField {
    fn default() -> Self {
        ActiveField::Amount
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            account_idx: 0,
            category_idx: 0,
            active_field: ActiveField::Amount,
            direction: DirectionKind::Expense,
            amount: String::new(),
            description: String::new(),
            to_account_idx: 0,
            new_account_name: String::new(),
            new_account_kind_idx: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActiveField {
    Amount,
    Description,
    AccountName,
    AccountKind,
}

#[derive(PartialEq, Eq)]
pub enum Mode {
    Normal,
    Input,
    Transfer,
    AddAccount,
    DeleteAccount,
}

pub struct App {
    pub backend_url: String,
    pub accounts: Vec<Account>,
    pub categories: Vec<Category>,
    pub transactions: Vec<Transaction>,
    pub status: String,
    pub mode: Mode,
    pub input: InputState,
}

impl App {
    pub fn new(backend_url: String) -> Self {
        Self {
            backend_url,
            accounts: Vec::new(),
            categories: Vec::new(),
            transactions: Vec::new(),
            status: "Press a add txn, t transfer, n new acct, x delete, q quit".to_string(),
            mode: Mode::Normal,
            input: InputState {
                direction: DirectionKind::Expense,
                ..Default::default()
            },
        }
    }
}
