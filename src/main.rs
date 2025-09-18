pub mod models;
pub mod params;
pub mod errors;
pub mod repositories;
pub mod use_cases;
pub mod utils;
pub mod cli;
pub mod config;
pub mod web;

#[tokio::main]
async fn main() {
    crate::cli::MainCli::run().await;
}
