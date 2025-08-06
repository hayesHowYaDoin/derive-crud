use derive_crud::Create;

#[derive(Create)]
#[crud_table("test_table")]
struct Test {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    Test::create("Debbie".to_string()).await;
}
