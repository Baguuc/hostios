impl crate::repositories::FilesRepository {
    /// # FilesRepository::move_
    ///
    /// move a file
    ///
    pub async fn move_<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::FileMoveParams,
        fql_client: &std::sync::Arc<crate::fql::Client>,
        client: A
    ) -> Result<(), FileMoveError> {
        type Error = FileMoveError; 
        
        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let statement = crate::fql::Statement::parse(format!("MOVE FILE {}, {};", params.path, params.new_path))
            .map_err(|_| Error::InvalidPath)?;

        let _ = fql_client.execute(statement)
            .await
            .map_err(|_| Error::CannotMove)?;
        
        let sql = "UPDATE file_tags SET file_path = $2 WHERE file_path = $1;";
        let _ = sqlx::query(sql)
            .bind(params.path)
            .bind(params.new_path)
            .execute(&mut *client)
            .await;

        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FileMoveError {
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("CANNOT_MOVE")]
    CannotMove,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
