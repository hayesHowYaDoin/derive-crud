use derive_crud::Read;

#[derive(Read)]
#[crud_table("test_table")]
#[crud_table("test_table_2")]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let _result = Test::read(1).await;
}
