impl crate::DirectoriesRepository {
    /// # DirectoriesRepository::move_
    ///
    /// move a directory to new path
    ///
    pub async fn move_(
        path: &crate::utils::Path,
        new_path: &crate::utils::Path,
        data_dir: &crate::utils::DataDirPath
    ) -> Result<(), ()> {
        let full_path_string = data_dir.join(path);
        let full_new_path_string = data_dir.join(new_path);
        
        std::fs::rename(full_path_string, full_new_path_string)
            .map_err(|_| ())?;
        
        return Ok(());
    }
}
