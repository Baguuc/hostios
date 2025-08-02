#[derive(serde::Deserialize)]
pub struct PathData {
   path: String
}

#[derive(serde::Deserialize)]
pub struct JsonData {
   new_path: String
}

#[actix_web::put("/entries/{path}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<PathData>,
    json: actix_web::web::Json<JsonData>,
    config: actix_web::web::Data<crate::config::Config>,
    entry_repository: actix_web::web::Data<hostios_application::EntryRepository>,
    _authios_sdk: actix_web::web::Data<authios_sdk::Sdk>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use std::path::PathBuf;
    use hostios_application::utils::Path;
    use hostios_application::EntryMoveError;
    use crate::utils::authios::authorize;
    
    if !authorize(_authios_sdk, &req, config.service_permission.clone()).await {
        return HttpResponse::Unauthorized().into();
    }

    let entry_path = match Path::parse(PathBuf::from(path.path.clone())) {
        Ok(entry_path) => entry_path,
        Err(_) => return HttpResponse::BadRequest().into()
    };
    
    let new_path = match Path::parse(PathBuf::from(json.new_path.clone())) {
        Ok(entry_path) => entry_path,
        Err(_) => return HttpResponse::BadRequest().into()
    };

    match entry_repository.move_entry(entry_path, new_path).await {
        Ok(_) => return HttpResponse::Ok().into(),
        Err(error) => return match error {
            EntryMoveError::NotExist => HttpResponse::NotFound().into(),
            EntryMoveError::NotAFile => HttpResponse::Conflict().into(),
            EntryMoveError::WrongPath => HttpResponse::BadRequest().into(),
            EntryMoveError::CannotMove => HttpResponse::InternalServerError().into()
        }
    };
}
