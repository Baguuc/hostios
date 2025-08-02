#[derive(serde::Deserialize)]
pub struct PathData {
   path: String,
   tag_name: String
}

#[actix_web::delete("/entries/{path}/tags/{tag_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<PathData>,
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

    let entry_path = match Path::parse(PathBuf::from(path.path.clone())) {
        Ok(entry_path) => entry_path,
        Err(_) => return HttpResponse::BadRequest().into()
    };

    match entry_repository.remove_tag(entry_path, path.tag_name.clone()).await {
        Ok(_) => return HttpResponse::Ok().into(),
        Err(_) => return HttpResponse::NotFound().into()
    };
}
