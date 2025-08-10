use derive_crud::Delete;

#[derive(Delete)]
#[crud_table("test_table")]
#[crud_table("test_table2")]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::SqlitePool::connect(&database_url).await.unwrap();

    let _result = Test::delete(&pool, 1).await;
}
