use super::model::{Account, Category, DirectionKind, Transaction};

#[derive(Clone)]
pub struct InputState {
    pub account_idx: usize,
    pub category_idx: usize,
    pub active_field: ActiveField,
    pub direction: DirectionKind,
    pub amount: String,
    pub description: String,
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
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ActiveField {
    Amount,
    Description,
}

#[derive(PartialEq, Eq)]
pub enum Mode {
    Normal,
    Input,
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
            status: "Press a to add, q to quit (live updates enabled)".to_string(),
            mode: Mode::Normal,
            input: InputState {
                direction: DirectionKind::Expense,
                ..Default::default()
            },
        }
    }
}
