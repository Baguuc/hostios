impl crate::repositories::FilesRepository {
    /// # FilesRepository::read
    ///
    /// read a file
    ///
    pub async fn read(
        params: crate::params::repository::FileReadParams 
    ) -> Result<String, crate::errors::repository::FileReadError> {
        use crate::utils::path::join_paths;
        type Error = crate::errors::repository::FileReadError;

        let full_path = join_paths(
            params.system_base_path,
            params.internal_path
        );
        let read = std::fs::read_to_string(full_path)
            .map_err(|_| Error::NotFound)?;
        
        return Ok(read);
    }
}
