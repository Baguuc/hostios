impl crate::DirectoriesUseCase {
    /// # DirectoriesUseCase::create
    ///
    /// check if user has permission to use this function ("hostios:tags:create"), then try to
    /// create a directory checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the directory already exist;
    /// + when the database connection cannot be acquired;
    ///
    pub async fn create(
        params: &DirectoryCreateParams, 
        _authios_sdk: authios_sdk::Sdk 
    ) -> Result<(), DirectoryCreateError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = DirectoryCreateError;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:tags:create")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        crate::DirectoriesRepository::create(&params.path, &params.data_dir)
            .await
            .map_err(|_| Error::AlreadyExist)?;

        return Ok(());
    }
}

pub struct DirectoryCreateParams {
    path: crate::utils::Path,
    data_dir: crate::utils::DataDirPath,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryCreateError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("ALREADY_EXIST")]
    AlreadyExist
}
