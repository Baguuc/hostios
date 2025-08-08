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
        _authios_sdk: authios_sdk::Sdk 
    ) -> Result<String, FileReadError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = FileReadError;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:files:read")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        // won't error as the path checks if the file exists
        let data = crate::FilesRepository::read(&params.file_path, &params.data_dir)
            .await
            .unwrap();

        return Ok(data);
    }
}

pub struct FileReadParams {
    file_path: crate::utils::Path,
    data_dir: crate::utils::DataDirPath,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum FileReadError {
    #[error("UNAUTHORIZED")]
    Unauthorized
}
