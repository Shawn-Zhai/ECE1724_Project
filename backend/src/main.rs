use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::FromRow;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::signal;
use tracing::{info, Level};
use uuid::Uuid;

type AppResult<T> = Result<Json<T>, (StatusCode, String)>;

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
enum AccountKind {
    Checking,
    Savings,
    Credit,
    Cash,
}

impl AccountKind {
    fn as_str(&self) -> &'static str {
        match self {
            AccountKind::Checking => "checking",
            AccountKind::Savings => "savings",
            AccountKind::Credit => "credit",
            AccountKind::Cash => "cash",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
enum TransactionDirection {
    Income,
    Expense,
    Transfer,
}

impl TransactionDirection {
    fn as_str(&self) -> &'static str {
        match self {
            TransactionDirection::Income => "income",
            TransactionDirection::Expense => "expense",
            TransactionDirection::Transfer => "transfer",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
struct Account {
    id: String,
    name: String,
    kind: String,
    balance: f64,
    created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
struct Category {
    id: String,
    name: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Transaction {
    id: String,
    account_id: String,
    amount: f64,
    direction: TransactionDirection,
    description: Option<String>,
    occurred_at: String,
    splits: Vec<TransactionSplit>,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
struct TransactionRow {
    id: String,
    account_id: String,
    amount: f64,
    direction: String,
    description: Option<String>,
    occurred_at: String,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
struct TransactionSplit {
    transaction_id: String,
    category_id: String,
    amount: f64,
}

#[derive(Deserialize)]
struct CreateAccount {
    name: String,
    kind: AccountKind,
}

#[derive(Deserialize)]
struct CreateCategory {
    name: String,
}

#[derive(Deserialize)]
struct SplitInput {
    category_id: String,
    amount: f64,
}

#[derive(Deserialize)]
struct CreateTransaction {
    account_id: String,
    amount: f64,
    direction: TransactionDirection,
    description: Option<String>,
    occurred_at: Option<String>,
    splits: Option<Vec<SplitInput>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_env_filter("info")
        .init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://finance.db".to_string());
    let pool = build_pool(&database_url).await?;
    init_db(&pool).await?;
    seed_defaults(&pool).await?;

    let state = AppState { pool };

    let app = Router::new()
        .route("/health", get(health))
        .route("/accounts", get(list_accounts).post(create_account))
        .route("/categories", get(list_categories).post(create_category))
        .route("/transactions", get(list_transactions).post(create_transaction))
        .route("/transactions/{id}", get(get_transaction))
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    info!("Backend running at http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async { signal::ctrl_c().await.expect("failed to install Ctrl+C handler") };
    ctrl_c.await;
    info!("signal received, shutting down");
}

async fn health() -> &'static str {
    "ok"
}

async fn list_accounts(State(state): State<AppState>) -> AppResult<Vec<Account>> {
    let rows = sqlx::query_as::<_, Account>(
        r#"
        SELECT
            a.id,
            a.name,
            a.kind,
            CAST(
                COALESCE(
                    SUM(
                        CASE t.direction
                            WHEN 'income' THEN t.amount
                            WHEN 'expense' THEN -t.amount
                            ELSE 0
                        END
                    ),
                    0
                ) AS REAL
            ) AS balance,
            a.created_at
        FROM accounts a
        LEFT JOIN transactions t ON t.account_id = a.id
        GROUP BY a.id
        ORDER BY a.created_at DESC
        "#,
    )
        .fetch_all(&state.pool)
        .await
        .map_err(internal_error)?;
    Ok(Json(rows))
}

async fn create_account(
    State(state): State<AppState>,
    Json(payload): Json<CreateAccount>,
) -> AppResult<Account> {
    let id = Uuid::new_v4().to_string();
    let now = OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap();
    sqlx::query(
        "INSERT INTO accounts (id, name, kind, balance, created_at) VALUES (?1, ?2, ?3, 0.0, ?4)",
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(payload.kind.as_str())
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(internal_error)?;

    let account = Account {
        id,
        name: payload.name,
        kind: payload.kind.as_str().to_string(),
        balance: 0.0,
        created_at: now,
    };
    Ok(Json(account))
}

async fn list_categories(State(state): State<AppState>) -> AppResult<Vec<Category>> {
    let rows = sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY name ASC")
        .fetch_all(&state.pool)
        .await
        .map_err(internal_error)?;
    Ok(Json(rows))
}

async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategory>,
) -> AppResult<Category> {
    let id = Uuid::new_v4().to_string();
    let now = OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap();
    sqlx::query(
        "INSERT INTO categories (id, name, created_at) VALUES (?1, ?2, ?3)",
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(&now)
    .execute(&state.pool)
    .await
    .map_err(|e| map_conflict(e, "category already exists"))?;

    let category = Category {
        id,
        name: payload.name,
        created_at: now,
    };
    Ok(Json(category))
}

async fn build_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    // Handle Windows absolute paths like sqlite://d:/path/finance.db by stripping the scheme
    // and feeding the remainder into filename(), which avoids URL parsing quirks.
    let opts = if database_url.starts_with("sqlite://") && !database_url.starts_with("sqlite::") {
        let path_str = database_url.trim_start_matches("sqlite://");
        let path = PathBuf::from(path_str);
        SqliteConnectOptions::default()
            .filename(path)
            .create_if_missing(true)
    } else {
        SqliteConnectOptions::from_str(database_url)?.create_if_missing(true)
    };

    SqlitePoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .max_connections(5)
        .connect_with(opts)
        .await
        .map_err(anyhow::Error::from)
}

fn parse_direction(dir: &str) -> Result<TransactionDirection, (StatusCode, String)> {
    match dir {
        "income" => Ok(TransactionDirection::Income),
        "expense" => Ok(TransactionDirection::Expense),
        "transfer" => Ok(TransactionDirection::Transfer),
        _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "invalid direction".into())),
    }
}

async fn init_db(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(pool)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS accounts (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            kind TEXT NOT NULL,
            balance REAL NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            id TEXT PRIMARY KEY,
            account_id TEXT NOT NULL,
            amount REAL NOT NULL,
            direction TEXT NOT NULL,
            description TEXT,
            occurred_at TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS transaction_splits (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            transaction_id TEXT NOT NULL,
            category_id TEXT NOT NULL,
            amount REAL NOT NULL,
            FOREIGN KEY (transaction_id) REFERENCES transactions(id) ON DELETE CASCADE,
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

fn internal_error<E: std::fmt::Display>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

fn map_conflict(err: sqlx::Error, message: &str) -> (StatusCode, String) {
    match err {
        sqlx::Error::Database(db_err) if db_err.message().contains("UNIQUE") => {
            (StatusCode::CONFLICT, message.to_string())
        }
        other => internal_error(other),
    }
}