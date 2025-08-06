use derive_crud::Delete;

#[derive(Delete)]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let _result = Test::delete(1).await;
}
