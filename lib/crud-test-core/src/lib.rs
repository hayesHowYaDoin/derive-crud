#[derive(Debug)]
pub struct TestDatabaseManager {
    database_path: String,
}

impl TestDatabaseManager {
    pub async fn new() -> Result<TestDatabaseManager, sqlx::Error> {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
        let database_path = database_url
            .strip_prefix("sqlite://")
            .expect("DATABASE_URL must start with 'sqlite://'");

        if std::path::Path::new(&database_path).exists() {
            std::fs::remove_file(&database_path).expect("Failed to delete existing database file");
        }
        std::fs::File::create(&database_path).expect("Failed to create new database file");

        let pool = sqlx::sqlite::SqlitePool::connect(&database_url)
            .await
            .expect("Failed to connect to the database");
        sqlx::query("CREATE TABLE test_table (id INTEGER PRIMARY KEY, name TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("Failed to create test_table");

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
