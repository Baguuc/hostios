impl crate::repositories::DirectoriesRepository {
    /// # DirectoriesRepository::move_
    ///
    /// move a directory
    ///
    pub async fn mv(
        params: crate::params::repository::DirectoryMoveParams
    ) -> Result<(), crate::errors::repository::DirectoryMoveError> {
        use crate::utils::path::join_paths;
        type Error = crate::errors::repository::DirectoryMoveError; 
        
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
