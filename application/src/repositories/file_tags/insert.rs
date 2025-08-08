impl crate::FileTagsRepository {
    /// # FileTagsRepository::insert
    ///
    /// insert a tag to file entry's tag list
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(path: &crate::utils::Path, tag: &String, client: A) -> Result<(), sqlx::Error> {
        let mut client = client.acquire()
            .await?;
        
        let path = path.to_string();
        let sql = "INSERT INTO file_tags (file_path, tag_name) VALUES ($1, $2);";

        sqlx::query(sql)
            .bind(path)
            .bind(tag)
            .execute(&mut *client)
            .await?;
        
        return Ok(());
    }
}
