use crud_macro_core::{
    parse_column_fields, parse_id_attribute, parse_struct_fields, parse_table_attribute,
};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

fn build_create_query(id_ident: &Ident, table: &str, attribute_idents: &[&Ident]) -> String {
    let insert_field_names = attribute_idents
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<_>>();

    let insert_placeholders = vec!["?"; attribute_idents.len()];

    let mut return_field_names: Vec<_> = vec![id_ident.to_string()];
    return_field_names.extend(attribute_idents.iter().map(|ident| ident.to_string()));

    format!(
        "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
        table,
        insert_field_names.join(", "),
        insert_placeholders.join(", "),
        return_field_names.join(", ")
    )
}

fn build_update_query(id_ident: &Ident, table: &str, attribute_idents: &[&Ident]) -> String {
    let set_clauses: Vec<_> = attribute_idents
        .iter()
        .map(|ident| format!("{ident} = ?"))
        .collect();

    format!(
        "UPDATE {} SET {} WHERE {} = ?",
        table,
        set_clauses.join(", "),
        id_ident,
    )
}

/// Implements a CRUD function to create items in the database.
///
/// Generates a function that inserts a new entry into the database table. The
/// new function will have the name `create`, and will take the non-ID fields
/// of the struct as parameters.
///
///
/// # Attributes
///
/// - `#[crud_id]`: Primary key for the database table. This attribute must be
///   placed on a single named field within the struct.
/// - `#[crud_table("table_name")]`: Name of the database table that the struct
///   is meant to represent. This attribute must be placed on the struct itself.
///
/// # Panics (Compile-time Errors)
///
/// This macro will cause a compile-time error if:
/// - It is applied to an enum or union (only structs are supported).
/// - The struct is not annotated with `#[crud_table("table_name")]`.
/// - A single field is not annotated with `#[crud_id]`.
/// - The struct does not accurately match the database schema for the table.
///
/// # Example
///
/// ```rust
/// use derive_crud::Create;
///
/// #[derive(Create)]
/// #[crud_table("users")]
/// struct User {
///    #[crud_id]
///   id: i64,
///   name: String,
///   email: String,
///   age: i32,
/// }
///
/// #[tokio::main]
/// async fn main() {
///    let result = Test::create("Debbie".to_string(), "debbie@hotmail.com", 47).await;
/// }
/// ```
#[proc_macro_derive(Create, attributes(crud_id, crud_table))]
pub fn create_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = parse_struct_fields!(input);

    let table_name = parse_table_attribute!(input);
    let id_ident = parse_id_attribute!(fields);
    let (column_idents, column_types) = parse_column_fields!(fields);

    let fn_params = column_idents
        .iter()
        .zip(column_types.iter())
        .map(|(ident, ty)| {
            quote! { #ident: #ty }
        });

    let query = build_create_query(id_ident, &table_name, &column_idents);

    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Creates a new entry in the database.
            ///
            /// The `#[crud_table("table_name")]` attribute specifies the database table to insert into.
            /// The field annotated with `#[crud_id]` is used as the identifier for the table.
            pub async fn create(pool: &::sqlx::Pool<::sqlx::Sqlite>, #(#fn_params),*) -> Result<Self, ::derive_crud::CRUDError> {
                let item = sqlx::query_as!(#struct_name, #query, #(#column_idents),*,)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| ::derive_crud::CRUDError(e.to_string()))?;

                Ok(item)
            }
        }
    }
    .into()
}

/// Implements a CRUD functions to read items from the database.
///
/// Generates functions that read one/multiple entries in the database table with a
/// given ID, or to read all entries at once.
///
///
/// # Attributes
///
/// - `#[crud_id]`: Primary key for the database table. This attribute must be
///   placed on a single named field within the struct.
/// - `#[crud_table("table_name")]`: Name of the database table that the struct
///   is meant to represent. This attribute must be placed on the struct itself.
///
/// # Panics (Compile-time Errors)
///
/// This macro will cause a compile-time error if:
/// - It is applied to an enum or union (only structs are supported).
/// - The struct is not annotated with `#[crud_table("table_name")]`.
/// - A single field is not annotated with `#[crud_id]`.
/// - The struct does not accurately match the database schema for the table.
///
/// # Example
///
/// ```rust
/// use derive_crud::Read;
///
/// #[derive(Read)]
/// #[crud_table("users")]
/// struct User {
///    #[crud_id]
///   id: i64,
///   name: String,
///   email: String,
///   age: i32,
/// }
///
/// #[tokio::main]
/// async fn main() {
///    let result = Test::read(1).await;
/// }
/// ```
#[proc_macro_derive(Read, attributes(crud_id, crud_table))]
pub fn read_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = parse_struct_fields!(input);

    let read_query = format!(
        "SELECT * FROM {} WHERE {} = ?",
        parse_table_attribute!(input),
        parse_id_attribute!(fields),
    );
    let read_one_query = format!(
        "SELECT * FROM {} WHERE {} = ?",
        parse_table_attribute!(input),
        parse_id_attribute!(fields),
    );
    let read_all_query = format!("SELECT * FROM {}", parse_table_attribute!(input),);

    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Reads entries from the database by their ID.
            ///
            /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
            /// The field annotated with `#[crud_id]` is used as the identifier for the table.
            pub fn read<'a>(
                pool: &'a ::sqlx::Pool<::sqlx::Sqlite>,
                id: i64
            ) -> ::std::pin::Pin<Box<impl ::futures_core::stream::Stream<Item = Result<#struct_name, ::derive_crud::CRUDError>> + 'a>> {
                use ::futures_util::StreamExt;

                Box::pin(async_stream::stream! {
                    let mut stream = ::sqlx::query_as!(#struct_name, #read_query, id).fetch(pool);
                    while let Some(item) = stream.next().await {
                        match item {
                            Ok(record) => yield Ok(record),
                            Err(e) => yield Err(::derive_crud::CRUDError(e.to_string())),
                        }
                    }
                })
            }

            /// Reads a single entry from the database by its ID.
            ///
            /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
            /// The field annotated with `#[crud_id]` is used as the identifier for the table.
            pub async fn read_one(pool: &::sqlx::Pool<::sqlx::Sqlite>, id: i64) -> Result<Self, ::derive_crud::CRUDError> {
                let item = sqlx::query_as!(#struct_name, #read_one_query, id)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| ::derive_crud::CRUDError(e.to_string()))?;

                Ok(item)
            }

            /// Reads all entries from the database.
            ///
            /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
            pub async fn read_all(pool: &::sqlx::Pool<::sqlx::Sqlite>) -> Result<::std::vec::Vec<Self>, ::derive_crud::CRUDError> {
                let items: Vec<#struct_name> = ::sqlx::query_as!(#struct_name, #read_all_query)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| ::derive_crud::CRUDError(e.to_string()))?;

                Ok(items)
            }
        }
    }
    .into()
}

/// Implements a CRUD function to read a single item from the database.
///
/// Generates a function that reads a single entry in the database table with a
/// given ID. If multiple rows exist in the table with the same ID, only one is
/// returned.The new function will have the name `read_one`, and will take the
/// ID field as a parameter.
///
/// # Attributes
///
/// - `#[crud_id]`: Primary key for the database table. This attribute must be
///   placed on a single named field within the struct.
/// - `#[crud_table("table_name")]`: Name of the database table that the struct
///   is meant to represent. This attribute must be placed on the struct itself.
///
/// # Panics (Compile-time Errors)
///
/// This macro will cause a compile-time error if:
/// - It is applied to an enum or union (only structs are supported).
/// - The struct is not annotated with `#[crud_table("table_name")]`.
/// - A single field is not annotated with `#[crud_id]`.
/// - The struct does not accurately match the database schema for the table.
///
/// # Example
///
/// ```rust
/// use derive_crud::ReadOne;
///
/// #[derive(ReadOne)]
/// #[crud_table("users")]
/// struct User {
///    #[crud_id]
///   id: i64,
///   name: String,
///   email: String,
///   age: i32,
/// }
///
/// #[tokio::main]
/// async fn main() {
///    let result = Test::read_one(1).await;
/// }
/// ```
#[proc_macro_derive(ReadOne, attributes(crud_id, crud_table))]
pub fn read_one_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = parse_struct_fields!(input);

    let query = format!(
        "SELECT * FROM {} WHERE {} = ?",
        parse_table_attribute!(input),
        parse_id_attribute!(fields),
    );

    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Reads a single entry from the database by its ID.
            ///
            /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
            /// The field annotated with `#[crud_id]` is used as the identifier for the table.
            pub async fn read_one(pool: &::sqlx::Pool<::sqlx::Sqlite>, id: i64) -> Result<Self, ::derive_crud::CRUDError> {
                let item = ::sqlx::query_as!(#struct_name, #query, id)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| ::derive_crud::CRUDError(e.to_string()))?;

                Ok(item)
            }
        }
    }
    .into()
}

/// Implements a CRUD function to read all items from the database.
///
/// Generates a function that reads all entries in the database table. The new
/// function will have the name `read_all`, and will take no parameters.
///
/// # Attributes
///
/// - `#[crud_id]`: Primary key for the database table. This attribute must be
///   placed on a single named field within the struct.
/// - `#[crud_table("table_name")]`: Name of the database table that the struct
///   is meant to represent. This attribute must be placed on the struct itself.
///
/// # Panics (Compile-time Errors)
///
/// This macro will cause a compile-time error if:
/// - It is applied to an enum or union (only structs are supported).
/// - The struct is not annotated with `#[crud_table("table_name")]`.
/// - A single field is not annotated with `#[crud_id]`.
/// - The struct does not accurately match the database schema for the table.
///
/// # Example
///
/// ```rust
/// use derive_crud::ReadAll;
///
/// #[derive(ReadAll)]
/// #[crud_table("users")]
/// struct User {
///    #[crud_id]
///   id: i64,
///   name: String,
///   email: String,
///   age: i32,
/// }
///
/// #[tokio::main]
/// async fn main() {
///    let result = Test::read_all().await;
/// }
/// ```
#[proc_macro_derive(ReadAll, attributes(crud_id, crud_table))]
pub fn read_all_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let query = format!("SELECT * FROM {}", parse_table_attribute!(input),);

    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Reads all entries from the database.
            ///
            /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
            pub async fn read_all(pool: &::sqlx::Pool<sqlx::Sqlite>) -> Result<::std::vec::Vec<Self>, ::derive_crud::CRUDError> {
                let items: ::std::vec::Vec<#struct_name> = ::sqlx::query_as!(#struct_name, #query)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| ::derive_crud::CRUDError(e.to_string()))?;

                Ok(items)
            }
        }
    }
    .into()
}

/// Implements a CRUD function to update an item in the database table..
///
/// Generates a function that updates an entry in the database table with the
/// current contents of the object. The new function will have the name
/// `update`, and will take no parameters.
///
/// # Attributes
///
/// - `#[crud_id]`: Primary key for the database table. This attribute must be
///   placed on a single named field within the struct.
/// - `#[crud_table("table_name")]`: Name of the database table that the struct
///   is meant to represent. This attribute must be placed on the struct itself.
///
/// # Panics (Compile-time Errors)
///
/// This macro will cause a compile-time error if:
/// - It is applied to an enum or union (only structs are supported).
/// - The struct is not annotated with `#[crud_table("table_name")]`.
/// - A single field is not annotated with `#[crud_id]`.
/// - The struct does not accurately match the database schema for the table.
///
/// # Example
///
/// ```rust
/// use derive_crud::Update;
///
/// #[derive(Update)]
/// #[crud_table("users")]
/// struct User {
///    #[crud_id]
///   id: i64,
///   name: String,
///   email: String,
///   age: i32,
/// }
///
/// #[tokio::main]
/// async fn main() {
///   let _result = User {
///       id: 1,
///       name: "Debbie".to_string(),
///       email: "debbie@hotmail.com",
///       age: 47,
///   }
///   .update()
///   .await;
/// }
/// ```
#[proc_macro_derive(Update, attributes(crud_id, crud_table))]
pub fn update_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = parse_struct_fields!(input);

    let table_name = parse_table_attribute!(input);
    let id_ident = parse_id_attribute!(fields);
    let (column_idents, _) = parse_column_fields!(fields);

    let query = build_update_query(id_ident, &table_name, &column_idents);

    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Writes the updated fields of the struct to the database.
            ///
            /// The `#[crud_table("table_name")]` attribute specifies the database table to update.
            /// The field annotated with `#[crud_id]` is used as the identifier for the table.
            pub async fn update(&self, pool: &::sqlx::Pool<::sqlx::Sqlite>) -> Result<(), ::derive_crud::CRUDError> {
                sqlx::query!(#query, self.#id_ident, #(self.#column_idents),*)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| ::derive_crud::CRUDError(e.to_string()))?;

                Ok(())
            }
        }
    }
    .into()
}

/// Implements a CRUD function to delete an item from the database table.
///
/// Generates a function that deletes an entry in the database table with a
/// provided ID. The new function will have the name `delete`, and will take
/// the struct's ID field as a parameter.
///
///
/// # Attributes
///
/// - `#[crud_id]`: Primary key for the database table. This attribute must be
///   placed on a single named field within the struct.
/// - `#[crud_table("table_name")]`: Name of the database table that the struct
///   is meant to represent. This attribute must be placed on the struct itself.
///
/// # Panics (Compile-time Errors)
///
/// This macro will cause a compile-time error if:
/// - It is applied to an enum or union (only structs are supported).
/// - The struct is not annotated with `#[crud_table("table_name")]`.
/// - A single field is not annotated with `#[crud_id]`.
/// - The struct does not accurately match the database schema for the table.
///
/// # Example
///
/// ```rust
/// use derive_crud::Delete;
///
/// #[derive(Delete)]
/// #[crud_table("users")]
/// struct User {
///    #[crud_id]
///   id: i64,
///   name: String,
///   email: String,
///   age: i32,
/// }
///
/// #[tokio::main]
/// async fn main() {
///   let _result = Test::delete(1).await;
/// }
/// ```
#[proc_macro_derive(Delete, attributes(crud_id, crud_table))]
pub fn delete_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let query = format!(
        "DELETE FROM {} WHERE {} = ?",
        parse_table_attribute!(input),
        parse_id_attribute!(parse_struct_fields!(input)),
    );

    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Deletes an entry from the database by its ID.
            ///
            /// The `#[crud_table("table_name")]` attribute specifies the database table to delete from.
            /// The field annotated with `#[crud_id]` is used as the identifier for the table.
            pub async fn delete(pool: &::sqlx::Pool<::sqlx::Sqlite>, id: i64) -> Result<(), ::derive_crud::CRUDError> {
                sqlx::query!(#query, id)
                    .execute(pool)
                    .await
                    .map_err(|e| ::derive_crud::CRUDError(e.to_string()))?;

                Ok(())
            }
        }
    }
    .into()
}
