pub struct TagRepository {
    db_client: sqlx::postgres::PgPool,
}

impl TagRepository {
    /// # TagRepository::new
    ///
    /// create a new TagRepository instance
    ///
    pub fn new(db_client: sqlx::postgres::PgPool) -> Self { 
        Self { db_client }
    }
    
    /// # TagRepository::select
    ///
    /// select a tag by name
    ///
    /// Errors:
    /// + when the tag with specified name do not exist;
    /// 
    pub async fn select(self: &Self, name: String) -> Result<hostios_domain::Tag, TagSelectError> {
        let sql = "SELECT name, description FROM tags WHERE name = $1;"; 
        
        let result = sqlx::query_as(sql)
            .bind(&name)
            .fetch_one(&self.db_client)
            .await
            .map_err(|_| TagSelectError::NotExist)?;
        
        return Ok(result);
    }

    /// TagRepository::insert 
    /// 
    /// insert a tag
    ///
    /// Errors:
    /// + when a tag with specified name already exists;
    ///
    pub async fn insert(self: &Self, name: String, description: String) -> Result<(), TagInsertError> {
        let sql = "INSERT INTO tags (name, description) VALUES ($1, $2);";

        let result = sqlx::query(sql)
            .bind(name)
            .bind(description)
            .execute(&self.db_client)
            .await
            .map_err(|_| TagInsertError::AlreadyExist)?;

        return Ok(());
    }

    /// TagRepository::delete
    ///
    /// delete a tag
    ///
    /// Errors:
    /// + when a tag with specified name do not exist;
    ///
    pub async fn delete(self: &Self, name: String) -> Result<(), TagDeleteError> {
        let sql = "DELETE FROM tags WHERE name = $1;";

        let result = sqlx::query(sql)
            .bind(name)
            .execute(&self.db_client)
            .await
            .unwrap();

        if result.rows_affected() == 0 {
            return Err(TagDeleteError::NotExist);
        }

        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TagSelectError {
    #[error("this tag do not exist")]
    NotExist
}

#[derive(thiserror::Error, Debug)]
pub enum TagInsertError {
    #[error("this tag already exist")]
    AlreadyExist
}

#[derive(thiserror::Error, Debug)]
pub enum TagDeleteError {
    #[error("this tag do not exist")]
    NotExist
}
