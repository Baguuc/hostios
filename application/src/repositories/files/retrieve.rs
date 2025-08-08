impl crate::FilesRepository {
    /// # FilesRepository::retrieve
    ///
    /// get file's metadata by the file path
    ///
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        path: &crate::utils::Path,
        client: A
    ) -> Result<hostios_domain::File, sqlx::Error> {
        let mut client = client.acquire().await?;
        
        let path_string = path
            .to_string();

        // query file's tags
        let sql = "SELECT t.name, t.description FROM file_tags ft INNER JOIN tags t ON ft.tag_name = t.name WHERE file_path = $1;";
        
        let tags = sqlx::query_as(sql)
            .bind(&path_string)
            .fetch_all(&mut *client)
            .await
            .unwrap_or(vec![]);

        let entry = hostios_domain::File {
            tags,
            full_path: path_string.clone()
        };
        
        return Ok(entry);
    }
}
