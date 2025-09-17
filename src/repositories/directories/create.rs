impl crate::repositories::DirectoriesRepository {
    /// # DirectoriesRepository::create
    ///
    /// create a directory
    ///
    pub async fn create(
        params: crate::params::repository::DirectoryCreateParams, 
        fql_client: &std::sync::Arc<crate::fql::Client>
    ) -> Result<(), DirectoryCreateError> {
        type Error = DirectoryCreateError; 
        
        let statement = crate::fql::Statement::parse(format!("CREATE DIR {};", params.path))
            .map_err(|_| Error::InvalidPath)?;

        fql_client.execute(statement)
            .await
            .map_err(|_| Error::AlreadyExist)?;

        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryCreateError {
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("ALREADY_EXIST")]
    AlreadyExist
}
