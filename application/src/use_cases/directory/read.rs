impl crate::DirectoriesUseCase {
    /// # DirectoriesUseCase::read
    ///
    /// check if user has permission to use this function ("hostios:directories:read), then try to
    /// read a directory, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the path is invalid;
    /// + when the directory do not exist;
    ///
    pub async fn read(
        params: &DirectoryReadParams, 
        _authios_sdk: authios_sdk::Sdk,
        fql_client: &fql::Client
    ) -> Result<Vec<hostios_domain::Entry>, DirectoryReadError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = DirectoryReadError;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:directories:read")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let statement = fql::Statement::parse(format!("READ DIR {};", params.path))
            .map_err(|_| Error::InvalidPath)?;

        let result = fql_client.execute(statement)
            .await
            .map_err(|_| Error::NotExist)?;

        return Ok(result.unwrap_entry_list());
    }
}

pub struct DirectoryReadParams {
    path: String,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryReadError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist,
}
