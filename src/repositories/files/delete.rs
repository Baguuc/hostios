impl crate::repositories::FilesRepository {
    /// # FilesRepository::delete
    ///
    /// delete a file
    ///
    pub async fn delete(
        params: crate::params::repository::FileDeleteParams 
    ) -> Result<(), crate::errors::repository::FileDeleteError> {
        use crate::utils::path::join_paths;
        type Error = crate::errors::repository::FileDeleteError;

        let full_path = join_paths(
            params.base_system_path,
            params.internal_path
        );

        std::fs::remove_file(full_path).map_err(|error| match error.kind() {
            std::io::ErrorKind::NotFound => Error::NotFound,
            _ => panic!("An error that shouldn't happen occured. Check server configuration and base data directory system permissions.")
        })?;

        return Ok(());
    }
}
