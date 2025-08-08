impl crate::DirectoriesUseCase {
    /// # DirectoriesUseCase::delete
    ///
    /// check if user has permission to use this function ("hostios:files:delete"), then try to
    /// delete a file, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the directory is not empty;
    /// + when database connection cannot be acquired;
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &DirectoryDeleteParams, 
        _authios_sdk: authios_sdk::Sdk
    ) -> Result<(), DirectoryDeleteError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = DirectoryDeleteError;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:files:delete")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        if crate::DirectoriesRepository::read(&params.file_path, &params.data_dir).await.len() > 0 {
            return Err(Error::NotEmpty);
        }

        let _ = crate::DirectoriesRepository::delete(&params.file_path, &params.data_dir)
            .await
            .map_err(|_| Error::CannotDelete)?;

        return Ok(());
    }
}

pub struct DirectoryDeleteParams {
    file_path: crate::utils::Path,
    data_dir: crate::utils::DataDirPath,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryDeleteError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("CANNOT_MOVE")]
    CannotDelete,
    #[error("NOT_EMPTY")]
    NotEmpty
}
