use std::io::{stdout, Stdout};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio::time::Duration;

use super::api::{refresh, submit_transaction};
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
    loop {
        terminal.draw(|f| ui(f, app))?;

        if !event::poll(Duration::from_millis(250))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            if app.mode == Mode::Normal {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => refresh(app).await?,
                    KeyCode::Char('a') => {
                        app.mode = Mode::Input;
                        app.status =
                            "Add transaction: type amount, Tab to switch fields, Enter to submit"
                                .into();
                    }
                    _ => {}
                }
            } else {
                handle_input_mode(key.code, app).await?;
            }
        }
    }
    Ok(())
}

pub async fn handle_input_mode(code: KeyCode, app: &mut App) -> Result<()> {
    match code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.status = "Cancelled".into();
        }
        KeyCode::Tab => {
            app.input.active_field = match app.input.active_field {
                ActiveField::Amount => ActiveField::Description,
                ActiveField::Description => ActiveField::Amount,
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
                app.input.account_idx =
                    (app.input.account_idx + 1) % app.accounts.len();
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
                app.input.category_idx =
                    (app.input.category_idx + 1) % app.categories.len();
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
        },
        KeyCode::Char(c) => match app.input.active_field {
            ActiveField::Amount => {
                if c.is_ascii_digit() || c == '.' || c == '-' {
                    app.input.amount.push(c);
                }
            }
            ActiveField::Description => {
                app.input.description.push(c);
            }
        },
        _ => {}
    }
    Ok(())
}
