use derive_crud::Delete;

#[derive(Delete)]
#[crud_table("test_table")]
struct Test {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let _result = Test::delete(1).await;
}
