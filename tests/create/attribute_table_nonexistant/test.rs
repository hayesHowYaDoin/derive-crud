use derive_crud::Create;

#[derive(Create)]
#[crud_table("nonexistant_table")]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::SqlitePool::connect(&database_url).await.unwrap();

    let _ = Test::create(&pool, "Debbie".to_string()).await;
}
