impl crate::FileTagsRepository { 
    /// # FileTagsRepository::list
    ///
    /// list a tag from file entry's tag list
    ///
    pub async fn list<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(tag: &String, client: A) -> Result<Vec<String>, sqlx::Error>  {
        let mut client = client.acquire().await?;
        
        let sql = "SELECT file_path FROM file_tags WHERE tag_name = $1;"; 

        let data: Vec<(String,)> = sqlx::query_as(sql)
            .bind(tag)
            .fetch_all(&mut *client)
            .await?;
        let data = data
            .iter()
            .map(|row| row.0.clone())
            .collect::<Vec<String>>();

        return Ok(data);
    }
}
