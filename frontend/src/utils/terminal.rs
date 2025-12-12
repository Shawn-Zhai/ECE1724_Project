use std::io::{Stdout, stdout};

use anyhow::Result;
use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use futures_util::{SinkExt, StreamExt};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};
use tokio_tungstenite::connect_async;

use super::api::{create_account, delete_account, refresh, submit_transaction};
use super::app::{ActiveField, App, Mode};
use super::ui::ui;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

pub async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> Result<()> {
    let (ws_tx, mut ws_rx) = mpsc::unbounded_channel();
    let events_url = format!(
        "{}/events",
        app.backend_url
            .replace("http://", "ws://")
            .replace("https://", "wss://")
    );
    tokio::spawn(start_event_listener(events_url, ws_tx));

    loop {
        while ws_rx.try_recv().is_ok() {
            refresh(app).await?;
        }

        terminal.draw(|f| ui(f, app))?;

        if !event::poll(Duration::from_millis(250))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match app.mode {
                Mode::Normal => handle_normal_mode(key.code, app)?,
                Mode::Input => handle_transaction_mode(key.code, app).await?,
                Mode::Transfer => handle_transfer_mode(key.code, app).await?,
                Mode::AddAccount => handle_add_account_mode(key.code, app).await?,
                Mode::DeleteAccount => handle_delete_account_mode(key.code, app).await?,
            };
            if app.mode == Mode::Normal && matches!(key.code, KeyCode::Char('q')) {
                break;
            }
        }
    }
    Ok(())
}

fn handle_normal_mode(code: KeyCode, app: &mut App) -> Result<()> {
    match code {
        KeyCode::Char('q') => {}
        KeyCode::Char('a') => {
            app.mode = Mode::Input;
            app.input = Default::default();
            app.status =
                "Add transaction: amount/description, Tab switches fields, Enter to submit".into();
        }
        KeyCode::Char('t') => {
            app.mode = Mode::Transfer;
            app.input = Default::default();
            app.input.direction = super::model::DirectionKind::Transfer;
            app.status = "Transfer: left/right source, up/down destination, amount then Enter"
                .into();
        }
        KeyCode::Char('n') => {
            app.mode = Mode::AddAccount;
            app.input = Default::default();
            app.input.active_field = ActiveField::AccountName;
            app.status = "New account: type name, Tab to change type, Enter to save".into();
        }
        KeyCode::Char('x') => {
            app.mode = Mode::DeleteAccount;
            app.status = "Delete account: left/right to pick (defaults locked), Enter to delete, Esc to cancel".into();
        }
        _ => {}
    }
    Ok(())
}

async fn start_event_listener(url: String, tx: mpsc::UnboundedSender<()>) {
    loop {
        match connect_async(&url).await {
            Ok((stream, _)) => {
                let (mut write, mut read) = stream.split();
                // Send a ping to keep the connection alive on some servers.
                let _ = write
                    .send(tokio_tungstenite::tungstenite::Message::Ping(vec![]))
                    .await;

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(tokio_tungstenite::tungstenite::Message::Text(_)) => {
                            let _ = tx.send(());
                        }
                        Ok(tokio_tungstenite::tungstenite::Message::Ping(data)) => {
                            let _ = write
                                .send(tokio_tungstenite::tungstenite::Message::Pong(data))
                                .await;
                        }
                        Ok(_) => {}
                        Err(_) => break,
                    }
                }
            }
            Err(_) => {}
        }
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn handle_transaction_mode(code: KeyCode, app: &mut App) -> Result<()> {
    match code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.status = "Cancelled".into();
        }
        KeyCode::Tab => {
            app.input.active_field = match app.input.active_field {
                ActiveField::Amount => ActiveField::Description,
                _ => ActiveField::Amount,
            };
        }
        KeyCode::Left => {
            if !app.accounts.is_empty() {
                app.input.account_idx =
                    (app.input.account_idx + app.accounts.len() - 1) % app.accounts.len();
            }
        }
        KeyCode::Right => {
            if !app.accounts.is_empty() {
                app.input.account_idx = (app.input.account_idx + 1) % app.accounts.len();
            }
        }
        KeyCode::Up => {
            if !app.categories.is_empty() {
                app.input.category_idx =
                    (app.input.category_idx + app.categories.len() - 1) % app.categories.len();
            }
        }
        KeyCode::Down => {
            if !app.categories.is_empty() {
                app.input.category_idx = (app.input.category_idx + 1) % app.categories.len();
            }
        }
        KeyCode::Char('d') => {
            use super::model::DirectionKind;
            app.input.direction = match app.input.direction {
                DirectionKind::Expense => DirectionKind::Income,
                _ => DirectionKind::Expense,
            };
        }
        KeyCode::Enter => {
            submit_transaction(app).await?;
        }
        KeyCode::Backspace => match app.input.active_field {
            ActiveField::Amount => {
                app.input.amount.pop();
            }
            ActiveField::Description => {
                app.input.description.pop();
            }
            _ => {}
        },
        KeyCode::Char(c) => match app.input.active_field {
            ActiveField::Amount => {
                // Amounts are non-negative: allow digits and a single decimal point.
                if c.is_ascii_digit() || (c == '.' && !app.input.amount.contains('.')) {
                    app.input.amount.push(c);
                }
            }
            ActiveField::Description => {
                app.input.description.push(c);
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}

pub async fn handle_transfer_mode(code: KeyCode, app: &mut App) -> Result<()> {
    match code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.status = "Cancelled".into();
        }
        KeyCode::Tab => {
            app.input.active_field = match app.input.active_field {
                ActiveField::Amount => ActiveField::Description,
                _ => ActiveField::Amount,
            };
        }
        KeyCode::Left => {
            if !app.accounts.is_empty() {
                app.input.account_idx =
                    (app.input.account_idx + app.accounts.len() - 1) % app.accounts.len();
            }
        }
        KeyCode::Right => {
            if !app.accounts.is_empty() {
                app.input.account_idx = (app.input.account_idx + 1) % app.accounts.len();
            }
        }
        KeyCode::Up => {
            if !app.accounts.is_empty() {
                app.input.to_account_idx =
                    (app.input.to_account_idx + app.accounts.len() - 1) % app.accounts.len();
            }
        }
        KeyCode::Down => {
            if !app.accounts.is_empty() {
                app.input.to_account_idx = (app.input.to_account_idx + 1) % app.accounts.len();
            }
        }
        KeyCode::Enter => {
            submit_transaction(app).await?;
        }
        KeyCode::Backspace => match app.input.active_field {
            ActiveField::Amount => {
                app.input.amount.pop();
            }
            ActiveField::Description => {
                app.input.description.pop();
            }
            _ => {}
        },
        KeyCode::Char(c) => match app.input.active_field {
            ActiveField::Amount => {
                if c.is_ascii_digit() || (c == '.' && !app.input.amount.contains('.')) {
                    app.input.amount.push(c);
                }
            }
            ActiveField::Description => {
                app.input.description.push(c);
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}

pub async fn handle_add_account_mode(code: KeyCode, app: &mut App) -> Result<()> {
    const ACCOUNT_KINDS: [&str; 4] = ["checking", "savings", "credit", "investment"];
    match code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.status = "Cancelled".into();
        }
        KeyCode::Tab => {
            app.input.active_field = match app.input.active_field {
                ActiveField::AccountName => ActiveField::AccountKind,
                _ => ActiveField::AccountName,
            };
        }
        KeyCode::Left | KeyCode::Up => {
            if app.input.active_field == ActiveField::AccountKind {
                app.input.new_account_kind_idx =
                    (app.input.new_account_kind_idx + ACCOUNT_KINDS.len() - 1) % ACCOUNT_KINDS.len();
            }
        }
        KeyCode::Right | KeyCode::Down => {
            if app.input.active_field == ActiveField::AccountKind {
                app.input.new_account_kind_idx =
                    (app.input.new_account_kind_idx + 1) % ACCOUNT_KINDS.len();
            }
        }
        KeyCode::Enter => {
            let kind = ACCOUNT_KINDS[app.input.new_account_kind_idx];
            let name = app.input.new_account_name.clone();
            create_account(app, &name, kind).await?;
            if app.mode != Mode::Normal {
                app.mode = Mode::Normal;
            }
        }
        KeyCode::Backspace => {
            if app.input.active_field == ActiveField::AccountName {
                app.input.new_account_name.pop();
            }
        }
        KeyCode::Char(c) => {
            if app.input.active_field == ActiveField::AccountName {
                app.input.new_account_name.push(c);
            }
        }
        _ => {}
    }
    Ok(())
}

pub async fn handle_delete_account_mode(code: KeyCode, app: &mut App) -> Result<()> {
    match code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.status = "Cancelled".into();
        }
        KeyCode::Left => {
            if !app.accounts.is_empty() {
                app.input.account_idx =
                    (app.input.account_idx + app.accounts.len() - 1) % app.accounts.len();
            }
        }
        KeyCode::Right => {
            if !app.accounts.is_empty() {
                app.input.account_idx = (app.input.account_idx + 1) % app.accounts.len();
            }
        }
        KeyCode::Enter => {
            if let Some(account) = app.accounts.get(app.input.account_idx) {
                let id = account.id.clone();
                delete_account(app, &id).await?;
                app.mode = Mode::Normal;
            } else {
                app.status = "No account selected".into();
            }
        }
        _ => {}
    }
    Ok(())
}
