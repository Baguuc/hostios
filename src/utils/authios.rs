pub async fn authorize(
    _authios_sdk: actix_web::web::Data<authios_sdk::AuthiosSdk>,
    req: &actix_web::HttpRequest,
    permission: String
) -> bool {
    let auth_token = match req.headers().get("Authorization") {
        Some(token) => token.to_str().unwrap().to_string(),
        None => String::new()
    };
    
    let authorize_params = authios_sdk::params::UserSdkAuthorizeParams {
        token: auth_token,
        permission: permission
    };
    let user_sdk = _authios_sdk.user();

    return match user_sdk.authorize(authorize_params).await {
        Ok(true) => true,
        Err(_) | Ok(false) => false
    };
}
