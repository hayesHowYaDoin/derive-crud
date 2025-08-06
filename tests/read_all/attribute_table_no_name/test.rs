use derive_crud::ReadAll;

#[derive(ReadAll)]
#[crud_table]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let _results = Test::read_all().await.unwrap(); // Test should fail before unwrap
}
