impl crate::repositories::FileTagsRepository { 
    /// # FileTagsRepository::list_tags
    ///
    /// list tags associated with provided file path
    ///
    pub async fn list_tags<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(path: &crate::models::Path, client: A) -> Result<Vec<String>, sqlx::Error>  {
        let mut client = client.acquire().await?;
        
        let sql = "SELECT tag_name FROM file_tags WHERE file_path = $1;"; 

        let data: Vec<(String,)> = sqlx::query_as(sql)
            .bind(path.to_string())
            .fetch_all(&mut *client)
            .await?;
        let data = data
            .iter()
            .map(|row| row.0.clone())
            .collect::<Vec<String>>();

        return Ok(data);
    }
}
