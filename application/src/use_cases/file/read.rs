impl crate::FilesUseCase {
    /// # FilesUseCase::read
    ///
    /// check if user has permission to use this function ("hostios:files:read), then try to
    /// read a file, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    ///
    pub async fn read(
        params: &FileReadParams, 
        _authios_sdk: authios_sdk::Sdk,
        fql_client: &fql::Client
    ) -> Result<String, FileReadError> {
        use crate::repositories::files::read::FileReadError as RepoError;
        use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = FileReadError;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:files:read")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        let data = crate::FilesRepository::read(&params.file_path, fql_client)
            .await
            .map_err(|error| match error {
                RepoError::InvalidPath => Error::InvalidPath,
                RepoError::NotExist => Error::NotExist
            })?;

        return Ok(data);
    }
}

pub struct FileReadParams {
    file_path: String,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum FileReadError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("INVALID_PATH")]
    NotExist
}
