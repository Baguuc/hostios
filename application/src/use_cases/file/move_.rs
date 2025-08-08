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
        client: A
    ) -> Result<(), FileMoveError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
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

        let _ = crate::FilesRepository::move_(&params.file_path, &params.new_file_path, &params.data_dir, &mut *client)
            .await
            .map_err(|_| Error::CannotMove)?;

        return Ok(());
    }
}

pub struct FileMoveParams {
    file_path: crate::utils::Path,
    new_file_path: crate::utils::Path,
    data_dir: crate::utils::DataDirPath,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum FileMoveError {
    #[error("DatabaseConnection")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("CANNOT_MOVE")]
    CannotMove
}
