use derive_crud::Update;

#[derive(Update)]
#[crud_table("test_table")]
struct Test {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let _result = Test {
        id: 1,
        name: "Debbie".to_string(),
    }
    .update()
    .await;
}
