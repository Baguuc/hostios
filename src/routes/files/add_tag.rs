#[actix_web::post("/files/{path}/tags/{tag_name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<PathData>,
    _authios_sdk: actix_web::web::Data<authios_sdk::AuthiosSdk>,
    fql_client: actix_web::web::Data<crate::fql::Client>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use crate::use_cases::file_tag::{
        FileTagsUseCase,
        add::{
            FileTagAddError as Error
        }
    };
    use crate::params::use_case::FileTagAddParams as Params;
     
    let user_token = match req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(str) => str.to_string(),
            Err(_) => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
        },
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };
    
    let file_path = match crate::models::Path::parse(path.path.to_string()) {
        Ok(file_path) => file_path,
        Err(_) => return HttpResponse::BadRequest().body("INVALID_PATH")
    }; 
    let tag_name = path.tag_name.to_string();
    let params = Params {
        file_path,
        tag_name,
        user_token,
    };

    let mut database_client = match database_client.acquire().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
    };
    
    match FileTagsUseCase::add(&params, &_authios_sdk.into_inner(), &fql_client.into_inner(), &mut *database_client).await {
        Ok(_) => return HttpResponse::Ok().into(),
        Err(error) => return match error {
            Error::InvalidPath => HttpResponse::BadRequest().body(error.to_string()),
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
            Error::TagNotExist => HttpResponse::NotFound().body(error.to_string()),
            Error::AlreadyAdded => HttpResponse::Conflict().body(error.to_string()),
            Error::PathNotExist => HttpResponse::NotFound().body(error.to_string()),
        }
    };
}

#[derive(serde::Deserialize)]
pub struct PathData {
   path: String,
   tag_name: String
}
