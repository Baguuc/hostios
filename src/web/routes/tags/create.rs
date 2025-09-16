#[actix_web::post("")]
pub async fn controller(
    req: actix_web::HttpRequest,
    json: actix_web::web::Json<JsonData>,
    _authios_sdk: actix_web::web::Data<authios_sdk::AuthiosSdk>,
    database_client: actix_web::web::Data<sqlx::PgPool>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use crate::use_cases::tag::TagsUseCase;
    use crate::params::use_case::TagCreateParams as Params;
    use crate::errors::use_case::TagCreateError as Error;
     
    let user_token = match req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(str) => str.to_string(),
            Err(_) => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
        },
        None => return HttpResponse::Unauthorized().body("INVALID_TOKEN")
    };
    
    let params = Params {
        name: json.name().clone(),
        description: json.description().clone(),
        user_token
    };

    let mut database_client = match database_client.acquire().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError().body("DATABASE_CONNECTION")
    };
    
    match TagsUseCase::create(&params, &_authios_sdk.into_inner(), &mut *database_client).await {
        Ok(_) => return HttpResponse::Ok().into(),
        Err(error) => return match error {
            Error::Unauthorized => HttpResponse::Unauthorized().body(error.to_string()),
            Error::DatabaseConnection => HttpResponse::InternalServerError().body(error.to_string()),
            Error::AlreadyExist => HttpResponse::Conflict().body(error.to_string())
        }
    };
}

type JsonData = crate::models::Tag;
