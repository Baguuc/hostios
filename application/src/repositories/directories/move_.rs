impl crate::DirectoriesRepository {
    /// # DirectoriesRepository::move_
    ///
    /// move a directory
    ///
    pub async fn move_(
        path: &String, 
        new_path: &String, 
        fql_client: &fql::Client
    ) -> Result<(), DirectoryMoveError> {
        type Error = DirectoryMoveError; 

        let statement = fql::Statement::parse(format!("MOVE DIR {}, {};", path, new_path))
            .map_err(|_| Error::InvalidPath)?;

        let _ = fql_client.execute(statement)
            .await
            .map_err(|_| Error::CannotMove)?;

        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryMoveError {
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("CANNOT_MOVE")]
    CannotMove
}
