impl crate::TagsRepository {
    /// # TagsRepository::retrieve
    ///
    /// retrieve a tag by name
    ///
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(name: &String, client: A) -> Result<hostios_domain::Tag, sqlx::Error> {
        let mut client = client.acquire().await?;
        
        let sql = "SELECT name, description FROM tags WHERE name = $1;"; 
        
        return sqlx::query_as(sql)
            .bind(&name)
            .fetch_one(&mut *client)
            .await;
    }
}
