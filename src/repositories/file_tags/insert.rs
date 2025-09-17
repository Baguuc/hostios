impl crate::repositories::FileTagsRepository {
    /// # FileTagsRepository::insert
    ///
    /// insert a tag to file entry's tag list
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::FileTagInsertParams,
        client: A
    ) -> Result<(), sqlx::Error> {
        let mut client = client.acquire()
            .await?;
        
        let sql = "INSERT INTO file_tags (file_path, tag_name) VALUES ($1, $2);";

        sqlx::query(sql)
            .bind(params.path)
            .bind(params.tag_name)
            .execute(&mut *client)
            .await?;
        
        return Ok(());
    }
}
