impl crate::repositories::FileTagsRepository { 
    /// # FileTagsRepository::list_paths
    ///
    /// list file paths associated with provided tag
    ///
    pub async fn list_paths<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::FileTagListPathsParams,
        client: A
    ) -> Result<Vec<String>, sqlx::Error>  {
        let mut client = client.acquire().await?;
        
        let sql = "SELECT file_path FROM file_tags WHERE tag_name = $1;"; 

        let data: Vec<(String,)> = sqlx::query_as(sql)
            .bind(params.tag_name)
            .fetch_all(&mut *client)
            .await?;
        let data = data
            .iter()
            .map(|row| row.0.clone())
            .collect::<Vec<String>>();

        return Ok(data);
    }
}
