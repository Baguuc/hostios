impl crate::repositories::DirectoriesRepository {
    /// # DirectoriesRepository::delete
    ///
    /// delete a directory
    ///
    pub async fn delete(
        params: crate::params::repository::DirectoryDeleteParams 
    ) -> Result<(), crate::errors::repository::DirectoryDeleteError> {
        use crate::utils::path::join_paths;
        type Error = crate::errors::repository::DirectoryDeleteError;

        let full_path = join_paths(
            params.system_path,
            params.internal_path
        );

        std::fs::remove_dir(full_path).map_err(|error| match error.kind() {
            std::io::ErrorKind::NotFound => Error::NotFound,
            std::io::ErrorKind::DirectoryNotEmpty => Error::NotEmpty,
            _ => panic!("An error that shouldn't happen occured. Check server configuration and base data directory system permissions.")
        })?;

        return Ok(());
    }
}
