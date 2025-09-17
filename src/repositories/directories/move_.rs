impl crate::repositories::DirectoriesRepository {
    /// # DirectoriesRepository::move_
    ///
    /// move a directory
    ///
    pub async fn move_(
        params: crate::params::repository::DirectoryMoveParams,
        fql_client: &std::sync::Arc<crate::fql::Client>
    ) -> Result<(), DirectoryMoveError> {
        type Error = DirectoryMoveError; 

        let statement = crate::fql::Statement::parse(format!("MOVE DIR {}, {};", params.path, params.new_path))
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
