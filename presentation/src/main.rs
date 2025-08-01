pub mod error;
pub mod prelude;
pub mod cli;
pub mod config;

use crate::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    use clap::Parser;

    let cli = crate::cli::MainCli::parse();
    cli.execute();

    return Ok(());
}
