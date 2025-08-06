use derive_crud::Create;

#[derive(Create)]
#[crud_table("test_table")]
struct Test {
    #[crud_id]
    id: i64,
    #[crud_id]
    name: String,
}

#[tokio::main]
async fn main() {
    Test::create("Debbie".to_string()).await;
}
