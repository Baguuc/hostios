impl crate::FilesUseCase {
    /// # FilesUseCase::delete
    ///
    /// check if user has permission to use this function ("hostios:files:delete"), then try to
    /// delete a file, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the file cannot be deleted;
    /// + when database connection cannot be acquired;
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &FileDeleteParams, 
        _authios_sdk: authios_sdk::Sdk,
        client: A
    ) -> Result<(), FileDeleteError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = FileDeleteError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:directories:delete")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        let _ = crate::FilesRepository::delete(&params.file_path, &params.data_dir, &mut *client)
            .await
            .map_err(|_| Error::CannotDelete)?;

        return Ok(());
    }
}

pub struct FileDeleteParams {
    file_path: crate::utils::Path,
    data_dir: crate::utils::DataDirPath,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum FileDeleteError {
    #[error("DatabaseConnection")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("CANNOT_MOVE")]
    CannotDelete
}
