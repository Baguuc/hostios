impl crate::use_cases::FilesUseCase {
    /// # FilesUseCase::read
    ///
    /// check if user has permission to use this function ("hostios:files:read), then try to
    /// read a file, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    ///
    pub async fn read(
        params: &crate::params::use_case::FileReadParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::AuthiosSdk>,
        fql_client: &std::sync::Arc<crate::fql::Client>
    ) -> Result<String, FileReadError> {
        use crate::repositories::files::read::FileReadError as RepoError;
        use authios_sdk::params::UserSdkAuthorizeParams;
        
        type Error = FileReadError;

        let authorize_params = UserSdkAuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:files:read")
        };
        let user_sdk = _authios_sdk.user();

        match user_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        let data = crate::repositories::FilesRepository::read(&params.file_path, fql_client)
            .await
            .map_err(|error| match error {
                RepoError::InvalidPath => Error::InvalidPath,
                RepoError::NotExist => Error::NotExist
            })?;

        return Ok(data);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FileReadError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("INVALID_PATH")]
    NotExist
}
