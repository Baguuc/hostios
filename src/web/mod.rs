pub mod routes;

pub async fn run_server(config: crate::config::Config) -> Result<(), crate::errors::web::RunApiError> {
    use actix_web::{HttpServer,App,web::Data};
    use crate::utils::database::create_pool;
    
    let port = config.port;
    let pool = create_pool(config.database.clone()).await?;   
    let _authios_sdk = authios_sdk::AuthiosSdk::new(config.authios_url.clone())?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(crate::fql::Client::new(std::path::PathBuf::from(&config.data_dir))))
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(_authios_sdk.clone()))
            .app_data(Data::new(config.clone()))
            .service(routes::files::scope())
            .service(routes::directories::scope())
            .service(routes::tags::scope())
    });
    server.bind(("0.0.0.0", port))?
        .run()
        .await?;

    return Ok(());
}
