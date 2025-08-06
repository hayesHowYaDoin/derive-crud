use derive_crud::Delete;

#[derive(Delete)]
#[crud_table("test_table")]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
    location: String,
}

#[tokio::main]
async fn main() {
    let _result = Test::delete(1).await;
}
