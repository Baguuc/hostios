impl crate::repositories::DirectoriesRepository {
    /// # DirectoriesRepository::create
    ///
    /// create a directory while creating all parent directories as-needed.
    ///
    pub async fn create(
        params: crate::params::repository::DirectoryCreateParams 
    ) -> Result<(), crate::errors::repository::DirectoryCreateError> {
        use crate::utils::path::join_paths;
        type Error = crate::errors::repository::DirectoryCreateError;

        let full_path = join_paths(
            params.base_path,
            params.internal_path
        );
        
        std::fs::create_dir_all(full_path).map_err(|error| match error.kind() {
            std::io::ErrorKind::AlreadyExists => Error::AlreadyExist,
            _ => panic!("An error that shouldn't happen occured. Check server configuration and base data directory system permissions.")
        })?;

        return Ok(());
    }
}
