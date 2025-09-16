pub mod models;
pub mod repositories;
pub mod use_cases;
pub mod utils;
pub mod fql;
pub mod cli;
pub mod config;
pub mod routes;

#[tokio::main]
async fn main() {
    use clap::Parser;

    let cli = crate::cli::MainCli::parse();
    cli.execute();
}
