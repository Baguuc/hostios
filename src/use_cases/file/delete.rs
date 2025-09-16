impl crate::use_cases::FilesUseCase {
    /// # FilesUseCase::delete
    ///
    /// check if user has permission to use this function ("hostios:files:delete"), then try to
    /// delete a file, checking for errors
    ///
    /// Errors:
    /// + when database connection cannot be acquired;
    /// + when user is not authorized to use this function;
    /// + when the path is invalid;
    /// + when the file not exist;
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &FileDeleteParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::AuthiosSdk>,
        fql_client: &std::sync::Arc<crate::fql::Client>,
        client: A
    ) -> Result<(), FileDeleteError> {
        use crate::repositories::files::delete::FileDeleteError as RepoError;
        use authios_sdk::params::UserSdkAuthorizeParams;
        
        type Error = FileDeleteError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let authorize_params = UserSdkAuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:files:delete")
        };
        let user_sdk = _authios_sdk.user();

        match user_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        let _ = crate::repositories::FilesRepository::delete(&params.file_path, fql_client, &mut *client)
            .await
            .map_err(|error| match error {
                RepoError::InvalidPath => Error::InvalidPath,
                RepoError::DatabaseConnection => Error::DatabaseConnection,
                RepoError::CannotDelete => Error::NotExist
            })?;

        return Ok(());
    }
}

pub struct FileDeleteParams {
    pub file_path: String,
    pub user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum FileDeleteError {
    #[error("DatabaseConnection")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist
}
