impl crate::use_cases::TagsUseCase {
    /// # TagsUseCase::create
    ///
    /// check if user has permission to use this function ("hostios:tags:create"), then try to
    /// create a tag checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the tag with provided name already exist;
    /// + when the database connection cannot be acquired;
    ///
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &TagCreateParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::AuthiosSdk>, 
        client: A
    ) -> Result<(), TagCreateError> {
        pub use authios_sdk::params::UserSdkAuthorizeParams;
        
        type Error = TagCreateError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let authorize_params = UserSdkAuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:tags:create")
        };
        let user_sdk = _authios_sdk.user();

        match user_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        crate::repositories::TagsRepository::insert(
            &params.name,
            &params.description,
            &mut *client
        )
            .await
            .map_err(|_| Error::AlreadyExist)?;

        return Ok(());
    }
}

pub struct TagCreateParams {
    pub name: String,
    pub description: String,
    pub user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum TagCreateError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("ALREADY_EXIST")]
    AlreadyExist
}
