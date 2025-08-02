#[derive(serde::Deserialize)]
pub struct QueryData {
   path: String 
}

#[actix_web::get("/entry")]
pub async fn controller(
    req: actix_web::HttpRequest,
    query: actix_web::web::Query<QueryData>,
    config: actix_web::web::Data<crate::config::Config>,
    entry_repository: actix_web::web::Data<hostios_application::EntryRepository>,
    _authios_sdk: actix_web::web::Data<authios_sdk::Sdk>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use std::path::PathBuf;
    use hostios_application::utils::Path;
    use crate::utils::authios::authorize;
    
    if !authorize(_authios_sdk, &req, config.service_permission.clone()).await {
        return HttpResponse::Unauthorized().into();
    }

    let entry_path = match Path::parse(PathBuf::from(query.path.clone())) {
        Ok(entry_path) => entry_path,
        Err(_) => return HttpResponse::BadRequest().into()
    };

    match entry_repository.select(entry_path).await {
        Ok(entry) => return HttpResponse::Ok().json(entry),
        Err(_) => return HttpResponse::NotFound().into()
    };
}
