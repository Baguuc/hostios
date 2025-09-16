#[actix_web::put("/{path}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<PathData>,
    json: actix_web::web::Json<JsonData>,
    _authios_sdk: actix_web::web::Data<authios_sdk::AuthiosSdk>,
    fql_client: actix_web::web::Data<crate::fql::Client>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use crate::use_cases::directory::DirectoriesUseCase;
    use crate::params::use_case::DirectoryMoveParams as Params;
    use crate::errors::use_case::DirectoryMoveError as Error;
     
    let user_token = match req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(str) => str.to_string(),
            Err(_) => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
        },
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };
    
    let params = Params {
        path: path.path.clone(),
        new_path: json.new_path.clone(),
        user_token
    };

    match DirectoriesUseCase::move_(&params, &_authios_sdk.into_inner(), &fql_client.into_inner()).await {
        Ok(_) => return HttpResponse::Ok().into(),
        Err(error) => return match error {
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::InvalidPath => HttpResponse::BadRequest().body(error.to_string()),
            Error::NotExist => HttpResponse::NotFound().body(error.to_string()),
            Error::NewParentPathNotExist => HttpResponse::Conflict().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
struct PathData { path: String }

#[derive(serde::Deserialize)]
struct JsonData { new_path: String }
