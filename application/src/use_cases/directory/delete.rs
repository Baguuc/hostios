impl crate::DirectoriesUseCase {
    /// # DirectoriesUseCase::delete
    ///
    /// check if user has permission to use this function ("hostios:files:delete"), then try to
    /// delete a file, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the path is invalid;
    /// + when the directory not exist;
    /// + when the directory is not empty;
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &DirectoryDeleteParams, 
        _authios_sdk: authios_sdk::Sdk,
        fql_client: &fql::Client
    ) -> Result<(), DirectoryDeleteError> {
        use crate::repositories::directories::delete::DirectoryDeleteError as RepoDeleteError;
        use crate::repositories::directories::read::DirectoryReadError as RepoReadError;
        use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = DirectoryDeleteError;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:files:delete")
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
        
        let _ = crate::DirectoriesRepository::delete(&params.path, fql_client)
            .await
            .map_err(|error| match error {
                RepoDeleteError::InvalidPath => Error::InvalidPath,
                RepoDeleteError::CannotDelete => Error::NotEmpty,
            })?;

        return Ok(());
    }
}

pub struct DirectoryDeleteParams {
    path: String,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryDeleteError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist,
    #[error("NOT_EMPTY")]
    NotEmpty,
}
