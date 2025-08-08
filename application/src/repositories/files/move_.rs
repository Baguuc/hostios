impl crate::FilesRepository {
    /// # FilesRepository::move_
    ///
    /// move a file to new path
    ///
    pub async fn move_<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        path: &crate::utils::Path,
        new_path: &crate::utils::Path,
        data_dir: &crate::utils::DataDirPath,
        client: A
    ) -> Result<(), ()> {
        let mut client = client.acquire()
            .await
            .map_err(|_| ())?;
        
        let full_path_string = data_dir.join(path);
        let full_new_path_string = data_dir.join(new_path);
        
        std::fs::rename(full_path_string, full_new_path_string)
            .map_err(|_| ())?;
        
        let sql = "UPDATE file_tags SET file_path = $1 WHERE file_path = $2;";
        sqlx::query(sql)
            .bind(new_path.to_string())
            .bind(path.to_string())
            .execute(&mut *client);

        
        return Ok(());
    }
}
