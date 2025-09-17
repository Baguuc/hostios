impl crate::repositories::TagsRepository {
    /// TagsRepository::insert 
    /// 
    /// insert a tag
    ///
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::TagInsertParams,
        client: A
    ) -> Result<(), sqlx::Error> {
        let mut client = client.acquire().await?;

        let sql = "INSERT INTO tags (name, description) VALUES ($1, $2);";

        sqlx::query(sql)
            .bind(&params.name)
            .bind(&params.description)
            .execute(&mut *client)
            .await?;

        return Ok(());
    }

}
