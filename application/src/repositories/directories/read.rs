impl crate::DirectoriesRepository {
    /// # DirectoriesRepository::read
    ///
    /// read a directory
    ///
    pub async fn read(
        path: &String, 
        fql_client: &std::sync::Arc<fql::Client>
    ) -> Result<Vec<hostios_domain::Entry>, DirectoryReadError> {
        type Error = DirectoryReadError; 

        let statement = fql::Statement::parse(format!("READ DIR {};", path))
            .map_err(|_| Error::InvalidPath)?;

        let result = fql_client.execute(statement)
            .await
            .map_err(|_| Error::NotExist)?
            .unwrap_entry_list();

        return Ok(result);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryReadError {
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist
}
