impl crate::DirectoriesUseCase {
    /// # DirectoriesUseCase::create
    ///
    /// check if user has permission to use this function ("hostios:tags:create"), then try to
    /// create a directory checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the path is invalid;
    /// + when the directory already exist;
    /// + when the database connection cannot be acquired;
    ///
    pub async fn create(
        params: &DirectoryCreateParams, 
        _authios_sdk: authios_sdk::Sdk,
        fql_client: &fql::Client
    ) -> Result<(), DirectoryCreateError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = DirectoryCreateError;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:tags:create")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        let statement = fql::Statement::parse(format!("CREATE DIR {};", params.path))
            .map_err(|_| Error::InvalidPath)?;

        fql_client.execute(statement)
            .await
            .map_err(|_| Error::AlreadyExist)?;

        return Ok(());
    }
}

pub struct DirectoryCreateParams {
    path: String,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryCreateError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("ALREADY_EXIST")]
    AlreadyExist,
    #[error("INVALID_PATH")]
    InvalidPath
}
