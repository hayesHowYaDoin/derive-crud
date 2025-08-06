use derive_crud::Create;

#[derive(Create)]
#[crud_table("test_table")]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let _result = Test::create("Debbie".to_string()).await;
}
