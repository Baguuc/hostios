#[actix_web::get("/files/{path}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<PathData>,
    _authios_sdk: actix_web::web::Data<authios_sdk::AuthiosSdk>,
    fql_client: actix_web::web::Data<crate::fql::Client>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use crate::use_cases::file::{
        FilesUseCase,
        read::{
            FileReadError as Error
        }
    };
    use crate::params::use_case::file::FileReadParams as Params;
     
    let user_token = match req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(str) => str.to_string(),
            Err(_) => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
        },
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };

    let file_path = path.path.to_string();
    
    let params = Params {
        file_path,
        user_token,
    };
    
    match FilesUseCase::read(&params, &_authios_sdk.into_inner(), &fql_client.into_inner()).await {
        Ok(content) => return HttpResponse::Ok().body(content),
        Err(error) => return match error {
            Error::NotExist => HttpResponse::NotFound().body(error.to_string()),
            Error::InvalidPath => HttpResponse::BadRequest().body(error.to_string()),
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
pub struct PathData {
   path: String
}
