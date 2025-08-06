use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn database_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;

    quote! {
        #[tokio::test]
        #[serial_test::serial]
        async fn #name() {
            let _db_manager = crud_test_core::TestDatabaseManager::new().await.expect("Failed to initialize test database");
            #block
        }
    }.into()
}
