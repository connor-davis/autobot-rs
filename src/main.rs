use anyhow::Error;
use ui::app::initialize_ui;

pub mod config;
pub mod functions;
pub mod ui;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialize_ui().await?;

    Ok(())
}
