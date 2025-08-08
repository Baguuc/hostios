impl crate::FilesRepository {
    /// # FilesRepository::delete
    ///
    /// delete file by it's path
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(path: &crate::utils::Path, data_dir: &crate::utils::DataDirPath, client: A) -> Result<(), ()> {
        let mut client = client.acquire()
            .await
            .map_err(|_| ())?;

        let sql = "DELETE FROM file_tags WHERE file_path = $1;";
        sqlx::query(sql)
            .bind(path.to_string())
            .execute(&mut *client);
        
        let full_path_string = data_dir.join(path);

        std::fs::remove_file(full_path_string)
            .map_err(|_| ())?;
        
        return Ok(());
    }
}
