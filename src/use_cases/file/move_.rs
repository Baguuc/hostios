impl crate::use_cases::FilesUseCase {
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
        params: &crate::params::use_case::FileMoveParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::AuthiosSdk>,
        fql_client: &std::sync::Arc<crate::fql::Client>,
        client: A
    ) -> Result<(), crate::errors::use_case::FileMoveError> {
        use crate::repositories::files::move_::FileMoveError as RepoMoveError;
        use authios_sdk::params::UserSdkAuthorizeParams;
        
        type Error = crate::errors::use_case::FileMoveError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let authorize_params = UserSdkAuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:files:move")
        };
        let user_sdk = _authios_sdk.user();

        match user_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let statement = crate::fql::Statement::parse(format!("EXISTS {};", params.file_path))
            .map_err(|_| Error::InvalidPath)?;
        let exists = fql_client.execute(statement).await
            .map_err(|_| Error::InvalidPath)?
            .unwrap_bool();
        
        if !exists {
            return Err(Error::NotExist);
        }

        let _ = crate::repositories::FilesRepository::move_(&params.file_path, &params.new_file_path, fql_client, &mut *client)
            .await
            .map_err(|error| match error {
                RepoMoveError::InvalidPath => Error::InvalidPath,
                RepoMoveError::CannotMove => Error::NewParentNotExist,
                RepoMoveError::DatabaseConnection => Error::DatabaseConnection
            })?;

        return Ok(());
    }
}
