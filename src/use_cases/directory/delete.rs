impl crate::use_cases::DirectoriesUseCase {
    /// # DirectoriesUseCase::delete
    ///
    /// check if user has permission to use this function ("hostios:directories:delete"), then try to
    /// delete a file, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the path is invalid;
    /// + when the directory not exist;
    /// + when the directory is not empty;
    ///
    pub async fn delete(
        params: &crate::params::use_case::DirectoryDeleteParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::AuthiosSdk>,
        fql_client: &std::sync::Arc<crate::fql::Client>
    ) -> Result<(), crate::errors::use_case::DirectoryDeleteError> {
        use crate::repositories::directories::delete::DirectoryDeleteError as RepoDeleteError;
        use crate::repositories::directories::read::DirectoryReadError as RepoReadError;
        use authios_sdk::params::UserSdkAuthorizeParams;
        
        type Error = crate::errors::use_case::DirectoryDeleteError;

        let authorize_params = UserSdkAuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:directories:delete")
        };
        let user_sdk = _authios_sdk.user();

        match user_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let _ = crate::repositories::DirectoriesRepository::read(&params.path, fql_client)
            .await
            .map_err(|error| match error {
                RepoReadError::InvalidPath => Error::InvalidPath,
                RepoReadError::NotExist => Error::NotExist
            })?;
        
        let _ = crate::repositories::DirectoriesRepository::delete(&params.path, fql_client)
            .await
            .map_err(|error| match error {
                RepoDeleteError::InvalidPath => Error::InvalidPath,
                RepoDeleteError::CannotDelete => Error::NotEmpty,
            })?;

        return Ok(());
    }
}
