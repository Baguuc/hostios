impl crate::repositories::FilesRepository {
    /// # FilesRepository::delete
    ///
    /// delete a file
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::FileDeleteParams, 
        fql_client: &std::sync::Arc<crate::fql::Client>,
        client: A
    ) -> Result<(), FileDeleteError> {
        type Error = FileDeleteError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;
        
        let statement = crate::fql::Statement::parse(format!("DELETE FILE {};", params.path.clone()))
            .map_err(|_| Error::InvalidPath)?;

        fql_client.execute(statement)
            .await
            .map_err(|_| Error::CannotDelete)?;

        let sql = "DELETE FROM file_tags WHERE file_path = $1;";
        let _ = sqlx::query(sql)
            .bind(params.path.clone())
            .execute(&mut *client)
            .await;

        return Ok(());
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FileDeleteError {
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("CANNOT_DELETE")]
    CannotDelete,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
