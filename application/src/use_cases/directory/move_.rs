impl crate::DirectoriesUseCase {
    /// # DirectoriesUseCase::move_
    ///
    /// check if user has permission to use this function ("hostios:files:move), then try to
    /// move a directory, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the directory cannot be moved;
    ///
    pub async fn move_(
        params: &DirectoryMoveParams, 
        _authios_sdk: authios_sdk::Sdk
    ) -> Result<(), DirectoryMoveError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = DirectoryMoveError;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:directories:move")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        let _ = crate::DirectoriesRepository::move_(&params.file_path, &params.new_file_path, &params.data_dir)
            .await
            .map_err(|_| Error::CannotMove)?;

        return Ok(());
    }
}

pub struct DirectoryMoveParams {
    file_path: crate::utils::Path,
    new_file_path: crate::utils::Path,
    data_dir: crate::utils::DataDirPath,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryMoveError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("CANNOT_MOVE")]
    CannotMove
}
