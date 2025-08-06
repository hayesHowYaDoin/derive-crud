use derive_crud::ReadAll;

#[derive(ReadAll)]
#[crud_table("test_table")]
struct Test {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let _result = Test::read_all().await.unwrap();
}
