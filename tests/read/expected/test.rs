use derive_crud::{Create, Read};

#[derive(Create, Read)]
#[crud_table("test_table")]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let test = Test::create("Debbie".to_string())
        .await
        .expect("Failed to create entry");

    let _result = Test::read(test.id).await.unwrap();
}
