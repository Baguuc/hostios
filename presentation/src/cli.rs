use crate::prelude::*;

#[derive(clap::Parser)]
#[command(name = "hostios")]
#[command(bin_name = "hostios")]
#[command(about = "A remote drive API", long_about = None)]
pub enum MainCli {
    #[command(about = "Run the HTTP server", long_about = None)]
    Run(Args),
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
        };
    }
}

async fn run(args: Args) {
    use actix_web::{HttpServer, App, web::Data};
    use futures::executor::block_on;
    use clin::components::{header,success,error};
    use colored::Colorize;
    use hostios_application::*;
    use crate::config::Config;
    use crate::error::error_if_necessary;
    
    header("Running web server");

    let config = error_if_necessary(Config::read(args.clone().config.unwrap_or(String::from("./hostios.json"))));
    
    success(format!("Server starting on port {}", config.port.to_string().underline()));
    
    let server = HttpServer::new(move || {
        let config = error_if_necessary(Config::read(args.clone().config.unwrap_or(String::from("./hostios.json"))));
        let pool = error_if_necessary(block_on(create_pool(config.database.clone())));

        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(TagRepository::new(pool.clone())))
            .app_data(Data::new(DirectoryRepository::new(config.data_dir.clone(), pool.clone())))
            .app_data(Data::new(EntryRepository::new(config.data_dir.clone(), pool)))
    });

    let binded_server = match server.bind(("0.0.0.0", config.port.clone())) {
        Ok(server) => server,
        Err(_) => {
            error("Cannot bind to port", config.port);
            
            std::process::exit(1);
        }
    };

    let _ = binded_server.run().await;
}

async fn create_pool(config: crate::config::DatabaseConfig) -> Result<sqlx::postgres::PgPool> {
    use sqlx::postgres::PgPool;

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user,
        config.password,
        config.host,
        config.port,
        config.database
    );
    let pool = PgPool::connect(connection_string.as_str()).await?;

    return Ok(pool);
}
