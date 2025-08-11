impl crate::FilesRepository {
    /// # FilesRepository::read
    ///
    /// read a file
    ///
    pub async fn read(
        path: &String, 
        fql_client: &std::sync::Arc<fql::Client>
    ) -> Result<String, FileReadError> {
        type Error = FileReadError; 

        let statement = fql::Statement::parse(format!("READ FILE {};", path))
            .map_err(|_| Error::InvalidPath)?;

        let result = fql_client.execute(statement)
            .await
            .map_err(|_| Error::NotExist)?
            .unwrap_string();

        return Ok(result);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FileReadError {
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("NOT_EXIST")]
    NotExist
}
