#[actix_web::delete("/tags/{name}")]
pub async fn controller(
    req: actix_web::HttpRequest,
    path: actix_web::web::Path<PathData>,
    _authios_sdk: actix_web::web::Data<authios_sdk::AuthiosSdk>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use crate::use_cases::tag::{
        TagsUseCase,
        delete::{
            TagDeleteParams as Params,
            TagDeleteError as Error
        }
    };
     
    let user_token = match req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(str) => str.to_string(),
            Err(_) => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
        },
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };
    
    let params = Params {
        name: path.name.clone(),
        user_token
    };

    let mut database_client = match database_client.acquire().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
    };
    
    match TagsUseCase::delete(&params, &_authios_sdk.into_inner(), &mut *database_client).await {
        Ok(_) => return HttpResponse::Ok().into(),
        Err(error) => return match error {
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
            Error::NotExist => HttpResponse::Conflict().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
struct PathData { name: String }
