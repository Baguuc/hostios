impl crate::DirectoriesRepository {
    /// # DirectoriesRepository::delete
    ///
    /// delete a directory
    ///
    pub async fn delete(path: &crate::utils::Path, data_dir: &crate::utils::DataDirPath) -> Result<(), ()> {
        let full_path_string = data_dir.join(path); 

        std::fs::remove_dir(full_path_string)
            .map_err(|_| ())?;

        return Ok(());
    }
}
