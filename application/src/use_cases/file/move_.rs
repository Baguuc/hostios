impl crate::FilesUseCase {
    /// # FilesUseCase::move_
    ///
    /// check if user has permission to use this function ("hostios:files:move), then try to
    /// move a file, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the file cannot be moved;
    /// + when database connection cannot be acquired;
    ///
    pub async fn move_<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &FileMoveParams, 
        _authios_sdk: authios_sdk::Sdk,
        fql_client: &fql::Client,
        client: A
    ) -> Result<(), FileMoveError> {
        use crate::repositories::files::move_::FileMoveError as RepoMoveError;
        use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = FileMoveError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:files:move")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let statement = fql::Statement::parse(format!("EXISTS {};", path))
            .map_err(|_| Error::InvalidPath)?;
        let exists = fql_client.execute(statement).await
            .map_err(|_| Error::InvalidPath)?
            .unwrap_bool();
        
        if !exists {
            return Err(Error::NotExist);
        }

        let _ = crate::FilesRepository::move_(&params.file_path, &params.new_file_path, fql_client, &mut *client)
            .await
            .map_err(|error| match error {
                RepoMoveError::InvalidPath => Error::InvalidPath,
                RepoMoveError::CannotMove => Error::NewParentNotExist,
                RepoMoveError::DatabaseConnection => Error::DatabaseConnection
            })?;

        return Ok(());
    }
}

pub struct FileMoveParams {
    file_path: String,
    new_file_path: String,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum FileMoveError {
    #[error("DatabaseConnection")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist,
    #[error("NEW_PARENT_NOT_EXIST")]
    NewParentNotExist
}
