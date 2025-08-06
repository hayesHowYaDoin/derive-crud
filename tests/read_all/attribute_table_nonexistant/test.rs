use derive_crud::ReadAll;

#[derive(ReadAll)]
#[crud_table("nonexistant_table")]
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
