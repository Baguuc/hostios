impl crate::FilesRepository {
    /// # FilesRepository::read
    ///
    /// read file's content
    ///
    pub async fn read(path: &crate::utils::Path, data_dir: &crate::utils::DataDirPath) -> Result<String, ()> {
        let full_path = data_dir.join(path);
        let content = std::fs::read_to_string(full_path)
            .map_err(|_| ())?;

        return Ok(content);
    }
}
