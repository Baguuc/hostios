impl crate::repositories::DirectoriesRepository {
    /// # DirectoriesRepository::read
    ///
    /// read a directory
    ///
    pub async fn read(
        params: crate::params::repository::DirectoryReadParams
    ) -> Result<Vec<crate::models::Entry>, crate::errors::repository::DirectoryReadError> {
        use crate::utils::path::join_paths;
        use crate::models::{Entry,Directory,File};
        type Error = crate::errors::repository::DirectoryReadError; 
        
        let full_path = join_paths(
            params.system_base_path,
            params.internal_path
        );
        let entries = std::fs::read_dir(full_path).map_err(|_| Error::NotExist)?;
        let mut read = vec![];

        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            let path_string = path
                .to_string_lossy()
                .to_string();
            
            let entry = if path.is_dir() {
                Entry::Directory(Directory::new(path_string).unwrap())
            } else {
                Entry::File(File::new(path_string).unwrap())
            };

            read.push(entry);
        }

        return Ok(read);
    }
}
