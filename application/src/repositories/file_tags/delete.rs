impl crate::FileTagsRepository { 
    /// # FileTagsRepository::delete
    ///
    /// delete a tag from file entry's tag list
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(path: &crate::utils::Path, tag: &String, client: A) -> Result<(), sqlx::Error>  {
        let mut client = client.acquire().await?;
        
        let path = path.to_string();
        let sql = "DELETE FROM file_tags WHERE file_path = $1 AND tag_name = $2;"; 

        sqlx::query(sql)
            .bind(path)
            .bind(tag)
            .execute(&mut *client)
            .await?;

        return Ok(());
    }
}
