use derive_crud::ReadAll;

#[derive(ReadAll)]
#[crud_table("test_table")]
struct Test {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::SqlitePool::connect(&database_url).await.unwrap();

    let _results = Test::read_all(&pool).await;
}
