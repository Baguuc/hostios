impl crate::repositories::FilesRepository {
    /// # FilesRepository::read
    ///
    /// read a file
    ///
    pub async fn read(
        params: crate::params::repository::FileReadParams, 
        fql_client: &std::sync::Arc<crate::fql::Client>
    ) -> Result<String, FileReadError> {
        type Error = FileReadError; 

        let statement = crate::fql::Statement::parse(format!("READ FILE {};", params.path))
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
