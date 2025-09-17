impl crate::repositories::TagsRepository {
    /// TagsRepository::delete
    ///
    /// delete a tag
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: crate::params::repository::TagDeleteParams,
        client: A
    ) -> Result<(), sqlx::Error> {
        let mut client = client.acquire().await?;
        
        let sql = "DELETE FROM tags WHERE name = $1;";

        sqlx::query(sql)
            .bind(&params.name)
            .execute(&mut *client)
            .await?;

        return Ok(());
    }
}
