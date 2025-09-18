impl crate::repositories::FilesRepository {
    /// # FilesRepository::mv
    ///
    /// move a file
    ///
    pub async fn mv(
        params: crate::params::repository::FileMoveParams
    ) -> Result<(), crate::errors::repository::FileMoveError> {
        use crate::utils::path::join_paths;
        type Error = crate::errors::repository::FileMoveError; 
        
        let old_full_path = join_paths(
            params.base_system_path.clone(),
            params.old_internal_path
        );
        let new_full_path = join_paths(
            params.base_system_path,
            params.new_internal_path
        );
        
        std::fs::rename(old_full_path, new_full_path).map_err(|_| Error::NewPathParentNotExist)?;

        return Ok(());
    }
}
