#[derive(clap::Parser)]
#[command(name = "hostios")]
#[command(bin_name = "hostios")]
#[command(about = "A remote drive API", long_about = None)]
pub enum MainCli {
    #[command(about = "Run the HTTP server", long_about = None)]
    Run(Args),
    #[command(about = "Run migrations on the database")]
    Migrate(Args)
}

#[derive(clap::Args, Clone)]
pub struct Args {
    #[clap(long, short)]
    config: Option<String>
}

impl MainCli {
    pub fn execute(self) {
        use futures::executor::block_on;

        match self {
            Self::Run(args) => { block_on(run(args)); },
            Self::Migrate(args) => { block_on(migrate(args)); }
        };
    }
}

async fn run(args: Args) {
    use clin::components::{header,success};
    use colored::Colorize;
    use crate::config::Config;
    use crate::utils::error::error_if_necessary;
    
    migrate(args.clone()).await;
    println!("");
    
    header("Running web server");

    let config = error_if_necessary(Config::read(args.clone().config.unwrap_or(String::from("./hostios.json"))));
    
    success(format!("Server starting on port {}", config.port.to_string().underline()));
    
    error_if_necessary(crate::web::run_server(config).await);
}

async fn migrate(args: Args) {
    use clin::components::{header,success};
    use crate::config::Config;
    use crate::utils::error::error_if_necessary;
    
    let config = error_if_necessary(Config::read(args.config.unwrap_or(String::from("./hostios.json"))));
    let pool = error_if_necessary(crate::utils::database::create_pool(config.database.clone()).await);
     
    header("Migrating database");
    
    error_if_necessary(crate::utils::migrations::migrate(&pool).await);
    success("Migrated");
}
