pub struct DirectoryRepository {
    data_dir: String,
    db_client: sqlx::postgres::PgPool
}

impl DirectoryRepository {
    /// # DirectoryRepository::new
    ///
    /// create a new DirectoryRepository instance
    ///
    pub fn new(data_dir: String, db_client: sqlx::postgres::PgPool) -> Self { 
        Self { data_dir, db_client  }
    }

    /// # DirectoryRepository::read 
    /// 
    /// read files and subdirectories and their metadata from the disk and database
    ///
    /// Errors:
    /// + when the path do not exist;
    /// + when the path is invalid;
    ///
    pub async fn read(self: &Self, path: String) -> Result<hostios_domain::Directory, DirectoryReadError> {
        let path_string: String = path
            .try_into()
            .map_err(|_| DirectoryReadError::WrongPath)?;
        let full_path_string = format!("{}/{}", self.data_dir, path_string);
        
        let entry_repository = crate::EntryRepository::new(
            self.data_dir.clone(),
            self.db_client.clone()
        );

        let mut entries = vec![];

        for entry in std::fs::read_dir(full_path_string).map_err(|_| DirectoryReadError::NotExist)? {
            let entry = entry.unwrap();
            let path = std::fs::canonicalize(entry.path())
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            let path = path.
                trim_start_matches(format!("{}/", self.data_dir).as_str())
                .to_string();

            // it's a valid path as it is fetched
            let entry = entry_repository.select(crate::utils::Path::parse(std::path::PathBuf::from(path)).unwrap())
                .await
                // it exists as it was just fetched
                .unwrap();

            entries.push(entry);
        }

        let entries = entries;
        
        let directory = hostios_domain::Directory {
            full_path: path_string,
            entries
        };

        return Ok(directory);
    }
    
    /// # DirectoryRepository::create
    ///
    /// create a directory
    ///
    /// Errors:
    /// + when the path is invalid;
    /// + when the directory cannot be created;
    ///
    pub async fn create(self: &Self, path: crate::utils::Path) -> Result<(), DirectoryCreateError> {
        let path_string: String = path
            .try_into()
            .map_err(|_| DirectoryCreateError::WrongPath)?;
        let full_path_string = format!("{}/{}", self.data_dir, path_string);
        
        std::fs::create_dir(full_path_string).map_err(|_| DirectoryCreateError::CannotCreate)?;

        return Ok(());
    }

    /// # DirectoryRepository::delete
    ///
    /// delete a directory
    ///
    /// Errors:
    /// + when the path do not exist;
    /// + when the path is not a directory;
    /// + when the path is not empty;
    /// + when the directory cannot be deleted;
    ///
    pub async fn delete(self: &Self, path: crate::utils::Path) -> Result<(), DirectoryDeleteError> {
        use std::path::Path;

        let path_string: String = path
            .try_into()
            .map_err(|_| DirectoryDeleteError::WrongPath)?;
        let full_path_string = format!("{}/{}", self.data_dir, path_string);

        let path = Path::new(&full_path_string);
        
        if !path.exists() {
            return Err(DirectoryDeleteError::NotExist);
        }

        if !path.is_dir() {
            return Err(DirectoryDeleteError::NotADirectory);
        }

        if std::fs::read_dir(path).map_err(|_| DirectoryDeleteError::NotExist)?.count() > 0 {
            return Err(DirectoryDeleteError::NotEmpty);
        }

        std::fs::remove_dir(path)
            .map_err(|_| DirectoryDeleteError::CannotDelete)?;

        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryReadError {
    #[error("provided path do not exist")]
    NotExist,

    #[error("provided path is invalid")]
    WrongPath,
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryCreateError {
    #[error("provided path is invalid")]
    WrongPath,
    
    #[error("cannot create the directory")]
    CannotCreate,
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryDeleteError {
    #[error("provided path is invalid")]
    WrongPath,
    
    #[error("provided path is not a directory")]
    NotADirectory,
    
    #[error("provided path do not exist")]
    NotExist,
    
    #[error("provided directory is not empty")]
    NotEmpty,
    
    #[error("provided path cannot be deleted")]
    CannotDelete,
}
