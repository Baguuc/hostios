#[actix_web::put("/{path}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<PathData>,
    json: actix_web::web::Json<JsonData>,
    _authios_sdk: actix_web::web::Data<authios_sdk::AuthiosSdk>,
    fql_client: actix_web::web::Data<crate::fql::Client>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use crate::use_cases::file::FilesUseCase;
    use crate::params::use_case::file::FileMoveParams as Params;
    use crate::errors::use_case::FileMoveError as Error;
     
    let user_token = match req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(str) => str.to_string(),
            Err(_) => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
        },
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };

    let file_path = path.path.to_string();
    let new_file_path = json.new_path.to_string();
    
    let params = Params {
        file_path,
        new_file_path,
        user_token,
    };

    let mut database_client = match database_client.acquire().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
    };
    
    match FilesUseCase::move_(&params, &_authios_sdk.into_inner(), &fql_client.into_inner(), &mut *database_client).await {
        Ok(_) => return HttpResponse::Ok().into(),
        Err(error) => return match error {
            Error::NotExist => HttpResponse::NotFound().body(error.to_string()),
            Error::InvalidPath => HttpResponse::BadRequest().body(error.to_string()),
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
            Error::NewParentNotExist => HttpResponse::Conflict().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
pub struct PathData {
   path: String
}

#[derive(serde::Deserialize)]
pub struct JsonData {
   new_path: String
}
