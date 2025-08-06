use derive_crud::ReadAll;

#[derive(ReadAll)]
#[crud_table("test_table")]
#[crud_table("test_table_2")]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let _results = Test::read_all().await;
}
