impl crate::use_cases::DirectoriesUseCase {
    /// # DirectoriesUseCase::read
    ///
    /// check if user has permission to use this function ("hostios:directories:read), then try to
    /// read a directory, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the path is invalid;
    /// + when the directory do not exist;
    ///
    pub async fn read(
        params: &DirectoryReadParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::AuthiosSdk>,
        fql_client: &std::sync::Arc<crate::fql::Client>
    ) -> Result<Vec<crate::models::Entry>, DirectoryReadError> {
        use crate::repositories::directories::read::DirectoryReadError as RepoError;
        use authios_sdk::params::UserSdkAuthorizeParams;
        
        type Error = DirectoryReadError;

        let authorize_params = UserSdkAuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:directories:read")
        };
        let user_sdk = _authios_sdk.user();

        match user_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let result = crate::repositories::DirectoriesRepository::read(&params.path, fql_client)
            .await
            .map_err(|error| match error {
                RepoError::InvalidPath => Error::InvalidPath,
                RepoError::NotExist => Error::NotExist
            })?;

        return Ok(result);
    }
}

pub struct DirectoryReadParams {
    pub path: String,
    pub user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryReadError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist,
}
