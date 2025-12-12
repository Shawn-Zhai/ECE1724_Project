mod utils;

use anyhow::Result;
use utils::{App, refresh, restore_terminal, run_app, setup_terminal};

#[tokio::main]
async fn main() -> Result<()> {
    let backend_url =
        std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

    let mut app = App::new(backend_url);
    app.status = "Loading data...".into();
    refresh(&mut app).await?;

    let mut terminal = setup_terminal()?;
    let res = run_app(&mut terminal, &mut app).await;
    restore_terminal()?;

    if let Err(err) = res {
        eprintln!("Error: {err}");
    }
    Ok(())
}
