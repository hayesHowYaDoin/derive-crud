use derive_crud::ReadAll;

#[derive(ReadAll)]
struct Test {
    #[crud_id]
    id: i64,
    name: String,
    location: String,
}

#[tokio::main]
async fn main() {
    let _results = Test::read_all().await;
}
