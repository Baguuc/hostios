pub struct EntryRepository {
    data_dir: String,
    db_client: sqlx::postgres::PgPool
}

impl EntryRepository {
    /// # EntryRepository::new
    ///
    /// create a new EntryRepository instance
    ///
    pub fn new(data_dir: String, db_client: sqlx::postgres::PgPool) -> Self {
        let data_dir = data_dir
            .trim_end_matches('/')
            .to_string();
        
        Self { data_dir, db_client }
    }

    /// # EntryRepository::insert_tag
    ///
    /// insert a tag to file entry's tag list
    ///
    /// Errors:
    /// + when the path do not exist;
    /// + when the path is invalid;
    /// + when the path is not a file;
    /// + when the tag do not exist;
    /// + when the tag is already added to this path;
    ///
    pub async fn insert_tag(self: &Self, path: crate::utils::Path, tag: String) -> Result<(), EntryTagInsertError> {
        use std::path::Path; 
        
        // throw error if tag not exist, otherwise skip
        let _ = crate::TagRepository::new(self.db_client.clone())
            .select(tag.clone())
            .await
            .map_err(|_| EntryTagInsertError::TagNotExist)?;
        
        let path_string: String = path
            .try_into()
            .map_err(|_| EntryTagInsertError::WrongPath)?;

        let full_path_string = format!("{}/{}", self.data_dir, path_string);
        let path = Path::new(&full_path_string);
        if !path.exists() {
            return Err(EntryTagInsertError::NotExist);
        } 
        
        if !path.is_file() {
            return Err(EntryTagInsertError::NotAFile);
        } 
        
        let sql = "INSERT INTO file_tags (file_path, tag_name) VALUES ($1, $2);"; 

        let result = sqlx::query(sql)
            .bind(path_string)
            .bind(tag)
            .execute(&self.db_client)
            .await
            .map_err(|_| EntryTagInsertError::TagAlreadyAdded)?;
        
        return Ok(());
    }
    
    /// # EntryRepository::remove_tag
    ///
    /// remove a tag from file entry's tag list
    ///
    /// Errors:
    /// + when the path is not a file;
    /// + when tag do not exist;
    /// + when tag is not added yet;
    ///
    pub async fn remove_tag(self: &Self, path: String, tag: String) -> Result<(), EntryTagDeleteError>  {
        // throw error if tag not exist, otherwise skip
        let _ = crate::TagRepository::new(self.db_client.clone())
            .select(tag.clone())
            .await
            .map_err(|_| EntryTagDeleteError::TagNotExist)?;
        
        let sql = "DELETE FROM file_tags WHERE file_path = $1 AND tag_name = $2;"; 

        let result = sqlx::query(sql)
            .bind(tag)
            .execute(&self.db_client)
            .await
            .unwrap();

        if result.rows_affected() == 0 {
            return Err(EntryTagDeleteError::NotAddedYet);
        }

        return Ok(());
    }
    
    /// # EntryRepository::select
    ///
    /// Get file entry's tags by the file path
    ///
    /// Errors:
    /// + when the file do not exists;
    /// + when the path is not a file;
    /// + when the path is invalid;
    ///
    pub async fn select(self: &Self, path: crate::utils::Path) -> Result<hostios_domain::Entry, EntrySelectError> {
        let path_string: String = path
            .try_into()
            .map_err(|_| EntrySelectError::WrongPath)?;
        
        let path_string = format!("{}/{}", self.data_dir, path_string);
        
        let path = std::path::Path::new(&path_string);
        if !path.exists() {
            return Err(EntrySelectError::NotExist);
        } 
        
        if !path.is_file() {
            return Err(EntrySelectError::NotAFile);
        } 
        
        // query file's tags
        let sql = "SELECT name, description FROM file_tags ft INNER JOIN tags t ON ft.tag_name = t.name WHERE file_path = path;";

        let tags = sqlx::query_as(sql)
            .bind(&path_string)
            .fetch_all(&self.db_client)
            .await
            .unwrap_or(vec![]);

        let entry = hostios_domain::Entry {
            tags,
            full_path: path_string.clone(),
            entry_type: if path.is_file() { hostios_domain::EntryType::File } else { hostios_domain::EntryType::Directory }
        };
        
        return Ok(entry);
    }
    
    /// # EntryRepository::read
    ///
    /// Read file entry's content
    ///
    /// Errors:
    /// + when the file do not exist;
    /// + when the path is not a file;
    ///
    pub async fn read(self: &Self, path: crate::utils::Path) -> Result<String, EntryReadError> {
        let path_string: String = path
            .try_into()
            .map_err(|_| EntryReadError::WrongPath)?;
        
        let path_string = format!("{}/{}", self.data_dir, path_string);
        
        let path = std::path::Path::new(&path_string);
        if !path.exists() {
            return Err(EntryReadError::NotExist);
        }
        
        if !path.is_file() {
            return Err(EntryReadError::NotAFile);
        }

        let content = std::fs::read_to_string(path)
            .map_err(|_| EntryReadError::NotExist)?;

        return Ok(content);
    }

    /// # EntryRepository::delete
    ///
    /// Read file entry's content
    ///
    /// Errors:
    /// + when the path do not exist;
    /// + when the path is not a file;
    /// + when the file cannot be deleted;
    ///
    pub async fn delete(self: &Self, path: crate::utils::Path) -> Result<(), EntryDeleteError> {
        let path_string: String = path
            .try_into()
            .map_err(|_| EntryDeleteError::WrongPath)?;
        
        let full_path_string = format!("{}/{}", self.data_dir, path_string);
        
        let path = std::path::Path::new(&full_path_string);
        if !path.exists() {
            return Err(EntryDeleteError::NotExist);
        }

        if !path.is_file() {
            return Err(EntryDeleteError::NotAFile);
        }

        std::fs::remove_file(path)
            .map_err(|_| EntryDeleteError::CannotDelete)?;
        
        let sql = "DELETE FROM file_tags WHERE path = $1";
        let _ = sqlx::query(sql)
            .bind(path_string)
            .execute(&self.db_client)
            .await;
        
        return Ok(());
    }
    
    /// # EntryRepository::move
    ///
    /// move a entry
    ///
    /// Errors:
    /// + when the file do not exist;
    /// + when the source path is not a file;
    /// + when the entry cannot be moved;
    /// 
    pub async fn move_entry(self: &Self, path: crate::utils::Path, new_path: crate::utils::Path) -> Result<(), EntryMoveError> {
        let path_string: String = path
            .try_into()
            .map_err(|_| EntryMoveError::WrongPath)?;
        let path_string = format!("{}/{}", self.data_dir, path_string);
        let path = std::path::Path::new(&path_string);

        if !path.exists() {
            return Err(EntryMoveError::NotExist);
        }
        
        if !path.is_file() {
            return Err(EntryMoveError::NotAFile);
        }

        let new_path_string: String = new_path
            .try_into()
            .map_err(|_| EntryMoveError::WrongPath)?;
        let new_path_string = format!("{}/{}", self.data_dir, new_path_string);
        let new_path = std::path::Path::new(&new_path_string);

        std::fs::rename(path, new_path)
            .map_err(|_| EntryMoveError::CannotMove)?;

        let sql = "UPDATE file_tags SET file_path = $1 WHERE file_path = $2";
        let _ = sqlx::query(sql)
            .bind(new_path_string)
            .bind(path_string)
            .execute(&self.db_client)
            .await;

        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EntryTagInsertError {
    #[error("provided tag do not exist")]
    NotExist,

    #[error("provided path is invalid")]
    WrongPath,
    
    #[error("provided path is not a file")]
    NotAFile,
    
    #[error("provided tag do not exist")]
    TagNotExist,
    
    #[error("tag already added")]
    TagAlreadyAdded   
}

#[derive(thiserror::Error, Debug)]
pub enum EntryTagDeleteError {
    #[error("provided tag is not added yet")]
    NotAddedYet,
    
    #[error("provided tag do not exist")]
    TagNotExist,
}

#[derive(thiserror::Error, Debug)]
pub enum EntrySelectError {
    #[error("provided tag do not exist")]
    NotExist,

    #[error("provided path is invalid")]
    WrongPath,
    
    #[error("not a file")]
    NotAFile
}

#[derive(thiserror::Error, Debug)]
pub enum EntryReadError {
    #[error("provided tag do not exist")]
    NotExist,

    #[error("provided path is invalid")]
    WrongPath,
    
    #[error("not a file")]
    NotAFile
}

#[derive(thiserror::Error, Debug)]
pub enum EntryDeleteError {
    #[error("provided path do not exist")]
    NotExist,

    #[error("provided path is invalid")]
    WrongPath,

    #[error("not a file")]
    NotAFile,
    
    #[error("cannot delete the file")]
    CannotDelete
}

#[derive(thiserror::Error, Debug)]
pub enum EntryMoveError {
    #[error("provided source do not exist")]
    NotExist,

    #[error("one of provided paths is invalid")]
    WrongPath,

    #[error("source is not a file")]
    NotAFile,
    
    #[error("cannot move the source file")]
    CannotMove
}
