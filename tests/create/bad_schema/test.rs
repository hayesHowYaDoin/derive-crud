use derive_crud::Create;

#[derive(Create)]
#[crud_table("test_table")]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
    location: String,
}

#[tokio::main]
async fn main() {
    Test::create("Debbie".to_string(), "New York".to_string())
        .await
        .expect("Failed to create entry");
}
