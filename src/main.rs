use anyhow::Error;
use functions::functions_handler::initialize_functions;
use ui::app::initialize_ui;

pub mod config;
pub mod functions;
pub mod ui;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tokio::spawn(initialize_functions());

    initialize_ui().await?;
    
    Ok(())
}
