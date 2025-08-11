#[actix_web::get("/files")]
pub async fn controller(
    req: actix_web::HttpRequest,
    query: actix_web::web::Query<QueryData>,
    _authios_sdk: actix_web::web::Data<authios_sdk::Sdk>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use hostios_application::use_cases::file_tag::{
        FileTagsUseCase,
        filter::{
            FileTagFilterParams as Params,
            FileTagFilterError as Error
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
        tag_name: query.tag.clone(),
        user_token
    };
    
    let mut database_client = match database_client.acquire().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
    };

    match FileTagsUseCase::filter(&params, &_authios_sdk.into_inner(), &mut *database_client).await {
        Ok(content) => return HttpResponse::Ok().json(content),
        Err(error) => return match error {
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
            Error::TagNotExist => HttpResponse::Conflict().body(error.to_string())
        }
    };
}

#[derive(serde::Deserialize)]
struct QueryData { tag: String }
