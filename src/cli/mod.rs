pub mod commands;

#[derive(clap::Parser)]
#[command(name = "hostios")]
#[command(bin_name = "hostios")]
#[command(about = "A remote drive API", long_about = None)]
pub enum MainCli {
    #[command(about = "Run the HTTP server", long_about = None)]
    Run(CliArgs),
    #[command(about = "Run migrations on the database")]
    Migrate(CliArgs)
}

#[derive(clap::Args, Clone)]
pub struct CliArgs {
    #[clap(long, short)]
    config: Option<String>
}

impl MainCli {
    pub async fn run() {
        use clap::Parser;

        let cli = Self::parse();
        cli.execute().await;
    }

    pub async fn execute(self) {
        match self {
            Self::Run(args) => { commands::run(args).await; },
            Self::Migrate(args) => { commands::migrate(args).await; }
        };
    }
}
