use derive_crud::Read;

#[derive(Read)]
#[crud_table("test_table")]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
    location: String,
}

#[tokio::main]
async fn main() {
    let _result = Test::read(1).await.unwrap(); // Test should fail before unwrap
}
