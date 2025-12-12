use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};

use super::app::{ActiveField, App, Mode};
use super::model::{Account, Category, DirectionKind, Transaction};

pub fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(f.area());

    let status = Paragraph::new(app.status.clone())
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[1]);

    render_accounts(f, main_chunks[0], &app.accounts);
    render_transactions(
        f,
        main_chunks[1],
        &app.transactions,
        &app.categories,
        &app.accounts,
    );

    render_input(f, chunks[2], app);
}

fn render_accounts(f: &mut ratatui::Frame, area: ratatui::layout::Rect, accounts: &[Account]) {
    let rows: Vec<Row> = accounts
        .iter()
        .map(|a| {
            Row::new(vec![
                Cell::from(a.name.clone()),
                Cell::from(a.kind.clone()),
                Cell::from(format!("{:.2}", a.balance)),
            ])
        })
        .collect();
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ],
    )
    .block(Block::default().title("Accounts").borders(Borders::ALL))
    .header(Row::new(vec!["Name", "Type", "Balance"]).style(Style::default().fg(Color::Yellow)))
    .column_spacing(1);
    f.render_widget(table, area);
}

fn render_transactions(
    f: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    txns: &[Transaction],
    categories: &[Category],
    accounts: &[Account],
) {
    let rows: Vec<Row> = txns
        .iter()
        .map(|t| {
            let account = accounts
                .iter()
                .find(|a| a.id == t.account_id)
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "unknown".into());
            let to_account = t
                .to_account_id
                .as_ref()
                .and_then(|id| accounts.iter().find(|a| a.id == *id))
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "-".into());
            let category = t
                .splits
                .first()
                .and_then(|s| categories.iter().find(|c| c.id == s.category_id))
                .map(|c| c.name.clone())
                .unwrap_or_else(|| "-".into());
            let signed_amount = match t.direction {
                DirectionKind::Income => t.amount,
                DirectionKind::Expense => -t.amount,
                DirectionKind::Transfer => t.amount,
            };
            Row::new(vec![
                Cell::from(account),
                Cell::from(format!("{:+.2}", signed_amount)),
                Cell::from(match t.direction {
                    DirectionKind::Income => "income",
                    DirectionKind::Expense => "expense",
                    DirectionKind::Transfer => "transfer",
                }),
                Cell::from(to_account),
                Cell::from(category),
                Cell::from(t.description.clone().unwrap_or_else(|| "".into())),
                Cell::from(t.occurred_at.clone()),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(14),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(13),
            Constraint::Percentage(18),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
        ],
    )
    .block(Block::default().title("Transactions").borders(Borders::ALL))
    .header(
        Row::new(vec![
            "Account",
            "Amount",
            "Dir",
            "To",
            "Category",
            "Description",
            "Date",
        ])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .column_spacing(1);

    f.render_widget(table, area);
}

fn render_input(f: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &App) {
    let mut lines = vec![Line::from(vec![
        Span::raw("Mode: "),
        Span::styled(
            match app.mode {
                Mode::Normal => "Normal",
                Mode::Input => "Adding",
                Mode::Transfer => "Transfer",
                Mode::AddAccount => "New Account",
                Mode::DeleteAccount => "Delete Account",
            },
            Style::default().fg(Color::Cyan),
        ),
        Span::raw(" | q quit | a add | t transfer | n new acct | x delete"),
    ])];

    if app.mode == Mode::Input {
        let account_name = app
            .accounts
            .get(app.input.account_idx)
            .map(|a| a.name.clone())
            .unwrap_or_else(|| "<no accounts>".into());
        let category_name = app
            .categories
            .get(app.input.category_idx)
            .map(|c| c.name.clone())
            .unwrap_or_else(|| "<no categories>".into());
        lines.push(Line::raw(format!(
            "Account: {} (left/right) | Category: {} (up/down)",
            account_name, category_name
        )));
        lines.push(Line::raw(format!(
            "Direction: {:?} (d to toggle)",
            app.input.direction
        )));

        let amount_style = if app.input.active_field == ActiveField::Amount {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let desc_style = if app.input.active_field == ActiveField::Description {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        lines.push(Line::from(vec![
            Span::styled(format!("Amount: {}", app.input.amount), amount_style),
            Span::raw(" | "),
            Span::styled(
                format!("Description: {}", app.input.description),
                desc_style,
            ),
            Span::raw(" | Tab switches fields | Enter to submit, Esc to cancel"),
        ]));
    } else if app.mode == Mode::Transfer {
        let from_name = app
            .accounts
            .get(app.input.account_idx)
            .map(|a| a.name.clone())
            .unwrap_or_else(|| "<no accounts>".into());
        let to_name = app
            .accounts
            .get(app.input.to_account_idx)
            .map(|a| a.name.clone())
            .unwrap_or_else(|| "<no accounts>".into());

        let amount_style = if app.input.active_field == ActiveField::Amount {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let desc_style = if app.input.active_field == ActiveField::Description {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        lines.push(Line::raw(format!(
            "From: {} (left/right) | To: {} (up/down)",
            from_name, to_name
        )));
        lines.push(Line::from(vec![
            Span::styled(format!("Amount: {}", app.input.amount), amount_style),
            Span::raw(" | "),
            Span::styled(
                format!("Description: {}", app.input.description),
                desc_style,
            ),
            Span::raw(" | Tab switches fields | Enter to submit, Esc to cancel"),
        ]));
    } else if app.mode == Mode::AddAccount {
        let name_style = if app.input.active_field == ActiveField::AccountName {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let kind_style = if app.input.active_field == ActiveField::AccountKind {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let kinds = ["checking", "savings", "credit", "investment"];
        let current_kind = kinds
            .get(app.input.new_account_kind_idx)
            .copied()
            .unwrap_or("checking");
        lines.push(Line::from(vec![
            Span::styled(
                format!("Name: {}", app.input.new_account_name),
                name_style,
            ),
            Span::raw(" | "),
            Span::styled(format!("Type: {}", current_kind), kind_style),
            Span::raw(" | Tab switches fields | Up/Down change type | Enter to save, Esc to cancel"),
        ]));
    } else if app.mode == Mode::DeleteAccount {
        let account_name = app
            .accounts
            .get(app.input.account_idx)
            .map(|a| a.name.clone())
            .unwrap_or_else(|| "<no accounts>".into());
        lines.push(Line::raw(format!(
            "Select account to delete (defaults locked): {} (left/right, Enter confirms, Esc cancels)",
            account_name
        )));
    }

    let paragraph =
        Paragraph::new(lines).block(Block::default().title("Controls").borders(Borders::ALL));
    f.render_widget(paragraph, area);
}
