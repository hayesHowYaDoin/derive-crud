use derive_crud::{Create};

#[derive(Create)]
#[crud_table("nonexistant_table")]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    Test::create("Debbie".to_string()).await.expect("Failed to create entry");
}
