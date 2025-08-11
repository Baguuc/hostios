impl crate::DirectoriesRepository {
    /// # DirectoriesRepository::delete
    ///
    /// delete a directory
    ///
    pub async fn delete(
        path: &String, 
        fql_client: &std::sync::Arc<fql::Client>
    ) -> Result<(), DirectoryDeleteError> {
        type Error = DirectoryDeleteError; 
        
        let statement = fql::Statement::parse(format!("DELETE DIR {};", path))
            .map_err(|_| Error::InvalidPath)?;

        fql_client.execute(statement)
            .await
            .map_err(|_| Error::CannotDelete)?;

        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryDeleteError {
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("CANNOT_DELETE")]
    CannotDelete
}
