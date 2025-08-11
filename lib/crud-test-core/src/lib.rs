use eyre::Result;

#[derive(Debug)]
pub struct TestDatabaseManager {
    database_path: String,
}

impl TestDatabaseManager {
    pub async fn new() -> Result<TestDatabaseManager> {
        let database_url = std::env::var("DATABASE_URL")?;
        let database_path = match database_url.strip_prefix("sqlite://") {
            Some(path) => path.to_string(),
            None => {
                return Err(eyre::eyre!("DATABASE_URL must start with 'sqlite://'"));
            }
        };

        if std::path::Path::new(&database_path).exists() {
            std::fs::remove_file(&database_path)?;
        }
        std::fs::File::create(&database_path)?;

        let pool = sqlx::sqlite::SqlitePool::connect(&database_url);
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        )
        .execute(&pool.await?)
        .await?;

        Ok(TestDatabaseManager {
            database_path: database_path.to_string(),
        })
    }
}

impl Drop for TestDatabaseManager {
    fn drop(&mut self) {
        if std::path::Path::new(&self.database_path).exists() {
            std::fs::remove_file(&self.database_path).expect("Failed to delete test database file");
        }
    }
}
