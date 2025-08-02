pub async fn authorize(
    _authios_sdk: actix_web::web::Data<authios_sdk::Sdk>,
    req: &actix_web::HttpRequest,
    permission: String
) -> bool {
    let auth_token = match req.headers().get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => String::new()
    };
    
    let authorize_params = authios_sdk::user::authorize::AuthorizeParams {
        token: auth_token,
        permission: permission
    };

    return match _authios_sdk.authorize(authorize_params).await {
        Ok(true) => true,
        Err(_) | Ok(false) => false
    };
}
