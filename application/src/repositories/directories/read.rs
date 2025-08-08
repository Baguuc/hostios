impl crate::DirectoriesRepository {
    /// # DirectoryRepository::read 
    /// 
    /// read files and subdirectories and their metadata from the disk and database
    ///
    pub async fn read(path: &crate::utils::Path, data_dir: &crate::utils::DataDirPath) -> Vec<crate::utils::Path> {
        let full_path_string = data_dir.join(path);
        let data_dir_path_string = data_dir.to_string();
        
        let content = std::fs::read_dir(full_path_string)
            .map_err(|_| ())?
            .into_iter()
            // it won't error
            .filter_map(|entry| {
                // the joined path will be absolute so we don't have to canonicalize
                let raw_path = entry
                    .unwrap()
                    .path()
                    .to_string_lossy()
                    .to_string()
                    .trim_start_matches(&data_dir_path_string)
                    .to_string();
                let pathbuf = std::path::PathBuf::from(raw_path);
                
                crate::utils::Path::parse(pathbuf, data_dir).ok()
            })
            .collect::<Vec<crate::utils::Path>>();
        
        return content;
    }
}
