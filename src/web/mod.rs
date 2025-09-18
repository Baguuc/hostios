pub mod routes;

pub async fn run_api(config: crate::config::Config) -> Result<(), crate::errors::web::RunApiError> {
    use actix_web::{HttpServer,App,web::Data};
    
    let port = config.port;
    let _authios_sdk = authios_sdk::AuthiosSdk::new(config.authios_url.clone())?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(_authios_sdk.clone()))
            .app_data(Data::new(config.clone()))
    });
    server.bind(("0.0.0.0", port))?
        .run()
        .await?;

    return Ok(());
}
