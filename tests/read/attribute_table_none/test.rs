use derive_crud::Read;

#[derive(Read)]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
    location: String,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::SqlitePool::connect(&database_url).await.unwrap();

    let _ = Test::read(&pool, 1);
}
