impl crate::repositories::DirectoriesRepository {
    /// # DirectoriesRepository::delete
    ///
    /// delete a directory
    ///
    pub async fn delete(
        params: crate::params::repository::DirectoryDeleteParams, 
        fql_client: &std::sync::Arc<crate::fql::Client>
    ) -> Result<(), DirectoryDeleteError> {
        type Error = DirectoryDeleteError; 
        
        let statement = crate::fql::Statement::parse(format!("DELETE DIR {};", params.path))
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
