impl crate::repositories::FileTagsRepository { 
    /// # FileTagsRepository::delete
    ///
    /// delete a tag from file entry's tag list
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::FileTagDeleteParams,
        client: A
    ) -> Result<(), sqlx::Error>  {
        let mut client = client.acquire().await?;
        
        let sql = "DELETE FROM file_tags WHERE file_path = $1 AND tag_name = $2;"; 

        sqlx::query(sql)
            .bind(&params.path)
            .bind(&params.tag_name)
            .execute(&mut *client)
            .await?;

        return Ok(());
    }
}
