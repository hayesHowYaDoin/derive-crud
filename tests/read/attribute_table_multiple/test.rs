use derive_crud::Read;

#[derive(Read)]
#[crud_table("test_table")]
#[crud_table("test_table_2")]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::SqlitePool::connect(&database_url).await.unwrap();

    let _ = Test::read(&pool, 1);
    let _ = Test::read_one(&pool, 1);
    let _ = Test::read_all(&pool);
}
