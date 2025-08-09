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
        
        let statement = fql::Statement::parse(format!("READ DIR {};", params.path))
            .map_err(|_| Error::InvalidPath)?;

        let dir_content = fql_client.execute(statement)
            .await
            .map_err(|_| Error::NotExist)?;

        if dir_content.unwrap_entry_list().len() > 0 {
            return Err(Error::NotEmpty);
        }

        let statement = fql::Statement::parse(format!("DELETE DIR {};", params.path))
            .map_err(|_| Error::InvalidPath)?;

        fql_client.execute(statement)
            .await;

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
