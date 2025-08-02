use derive_crud::{Create};

#[derive(Create)]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    Test::create("Debbie".to_string()).await.expect("Failed to create entry");
}
