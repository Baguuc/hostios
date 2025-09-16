impl crate::use_cases::DirectoriesUseCase {
    /// # DirectoriesUseCase::create
    ///
    /// check if user has permission to use this function ("hostios:directories:create"), then try to
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
        _authios_sdk: &std::sync::Arc<authios_sdk::AuthiosSdk>,
        fql_client: &std::sync::Arc<crate::fql::Client>
    ) -> Result<(), DirectoryCreateError> {
        use crate::repositories::directories::create::DirectoryCreateError as RepoError;
        use crate::repositories::DirectoriesRepository;
        use authios_sdk::params::UserSdkAuthorizeParams;
        
        type Error = DirectoryCreateError;

        let authorize_params = UserSdkAuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:directories:create")
        };
        let user_sdk = _authios_sdk.user();

        match user_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let _ = DirectoriesRepository::create(&params.path, fql_client)
            .await
            .map_err(|error| match error {
                RepoError::InvalidPath => Error::InvalidPath,
                RepoError::AlreadyExist => Error::AlreadyExist,
            });

        return Ok(());
    }
}

pub struct DirectoryCreateParams {
    pub path: String,
    pub user_token: String
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
