pub mod cli;
pub mod config;
pub mod routes;
pub mod utils;

#[tokio::main]
async fn main() {
    use clap::Parser;

    let cli = crate::cli::MainCli::parse();
    cli.execute();
}
