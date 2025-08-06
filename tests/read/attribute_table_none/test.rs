use derive_crud::Read;

#[derive(Read)]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
    location: String,
}

#[tokio::main]
async fn main() {
    let _result = Test::read(1).await;
}
