impl crate::use_cases::DirectoriesUseCase {
    /// # DirectoriesUseCase::move_
    ///
    /// check if user has permission to use this function ("hostios:directories:move), then try to
    /// move a directory, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the path is invalid;
    /// + when the directory cannot be moved;
    ///
    pub async fn move_(
        params: &crate::params::use_case::DirectoryMoveParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::AuthiosSdk>,
        fql_client: &std::sync::Arc<crate::fql::Client>
    ) -> Result<(), crate::errors::use_case::DirectoryMoveError> {
        use crate::repositories::directories::move_::DirectoryMoveError as RepoMoveError;
        use crate::repositories::directories::read::DirectoryReadError as RepoReadError;
        use crate::params::repository::{
            DirectoryReadParams,
            DirectoryMoveParams
        };
        use authios_sdk::params::UserSdkAuthorizeParams;
        
        type Error = crate::errors::use_case::DirectoryMoveError;

        let authorize_params = UserSdkAuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:directories:move")
        };
        let user_sdk = _authios_sdk.user();

        match user_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let _ = crate::repositories::DirectoriesRepository::read(DirectoryReadParams { path: params.path.clone() }, fql_client)
            .await
            .map_err(|error| match error {
                RepoReadError::InvalidPath => Error::InvalidPath,
                RepoReadError::NotExist => Error::NotExist
            })?;
        
        let _ = crate::repositories::DirectoriesRepository::move_(DirectoryMoveParams { path: params.path.clone(), new_path: params.new_path.clone() }, fql_client)
            .await
            .map_err(|error| match error {
                RepoMoveError::InvalidPath => Error::InvalidPath,
                RepoMoveError::CannotMove => Error::NewParentPathNotExist
            })?;

        return Ok(());
    }
}
