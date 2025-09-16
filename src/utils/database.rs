pub async fn create_pool(config: crate::config::DatabaseConfig) -> Result<sqlx::postgres::PgPool, sqlx::Error> {
    use sqlx::postgres::PgPool;

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user,
        config.password,
        config.host,
        config.port,
        config.database
    );
    let pool = PgPool::connect(connection_string.as_str()).await?;

    return Ok(pool);
}

const MIGRATIONS: [&str; 2] = [
"CREATE TABLE IF NOT EXISTS tags (
  name TEXT NOT NULL PRIMARY KEY,
  description TEXT NOT NULL DEFAULT ''
);",
"CREATE TABLE IF NOT EXISTS file_tags (
  file_path TEXT,
  tag_name TEXT NOT NULL,
  FOREIGN KEY (tag_name) REFERENCES tags(name) ON DELETE CASCADE,
  
  UNIQUE (file_path, tag_name)
);"
];

pub async fn migrate<'c, C: sqlx::Acquire<'c, Database = sqlx::Postgres>>(client: C) -> Result<(), sqlx::Error> {
    use sqlx::query;

    let mut client = client
        .acquire()
        .await?;

    for sql in MIGRATIONS {
        let _ = query(sql).execute(&mut *client).await?;
    }
    
    return Ok(());
}
