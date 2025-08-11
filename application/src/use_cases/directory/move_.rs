impl crate::DirectoriesUseCase {
    /// # DirectoriesUseCase::move_
    ///
    /// check if user has permission to use this function ("hostios:files:move), then try to
    /// move a directory, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the path is invalid;
    /// + when the directory cannot be moved;
    ///
    pub async fn move_(
        params: &DirectoryMoveParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::Sdk>,
        fql_client: &std::sync::Arc<fql::Client>
    ) -> Result<(), DirectoryMoveError> {
        use crate::repositories::directories::move_::DirectoryMoveError as RepoMoveError;
        use crate::repositories::directories::read::DirectoryReadError as RepoReadError;
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = DirectoryMoveError;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:directories:move")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let _ = crate::DirectoriesRepository::read(&params.path, fql_client)
            .await
            .map_err(|error| match error {
                RepoReadError::InvalidPath => Error::InvalidPath,
                RepoReadError::NotExist => Error::NotExist
            })?;
        
        let _ = crate::DirectoriesRepository::move_(&params.path, &params.new_path, fql_client)
            .await
            .map_err(|error| match error {
                RepoMoveError::InvalidPath => Error::InvalidPath,
                RepoMoveError::CannotMove => Error::NewParentPathNotExist
            })?;

        return Ok(());
    }
}

pub struct DirectoryMoveParams {
    pub path: String,
    pub new_path: String,
    pub user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryMoveError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist,
    #[error("NEW_PARENT_PATH_NOT_EXIST")]
    NewParentPathNotExist,
}
