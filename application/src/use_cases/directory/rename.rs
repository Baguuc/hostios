impl crate::DirectoriesUseCase {
    /// # DirectoriesUseCase::rename
    ///
    /// check if user has permission to use this function ("hostios:directories:move), then try to
    /// rename a directory, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the directory cannot be renamed;
    /// + when database connection cannot be acquired;
    ///
    pub async fn rename(
        params: &DirectoryRenameParams, 
        _authios_sdk: authios_sdk::Sdk
    ) -> Result<(), DirectoryRenameError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = DirectoryRenameError;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:directories:move")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        let pathbuf = std::path::PathBuf::from(params.file_path.to_string());
        let parent = pathbuf.parent().unwrap().to_string_lossy();

        let new_file_path = crate::utils::Path::parse(std::path::PathBuf::from(format!("{}/{}", parent, params.new_name)), &params.data_dir)
            .unwrap();
        
        let _ = crate::DirectoriesRepository::move_(&params.file_path, &new_file_path, &params.data_dir)
            .await
            .map_err(|_| Error::CannotRename)?;

        return Ok(());
    }
}

pub struct DirectoryRenameParams {
    file_path: crate::utils::Path,
    new_name: String,
    data_dir: crate::utils::DataDirPath,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryRenameError {
    #[error("DatabaseConnection")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("CANNOT_MOVE")]
    CannotRename
}
