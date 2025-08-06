use derive_crud::Read;

#[derive(Read)]
#[crud_table]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let _result = Test::read(1).await.unwrap(); // Test should fail before unwrap
}
