impl crate::DirectoriesRepository {
    /// # DirectoriesRepository::create
    ///
    /// create a directory
    ///
    pub async fn create(path: &crate::utils::Path, data_dir: &crate::utils::DataDirPath) -> Result<(), ()> {
        let full_path_string = data_dir.join(path);

        std::fs::create_dir(full_path_string)
            .map_err(|_| ())?;

        return Ok(());
    }
}
