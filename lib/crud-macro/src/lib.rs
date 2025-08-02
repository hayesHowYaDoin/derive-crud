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
    return_field_names.extend(attribute_idents.into_iter().map(|ident| ident.to_string()));

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
        .map(|ident| format!("{} = ?", ident))
        .collect();

    format!(
        "UPDATE {} SET {} WHERE {} = ?",
        table,
        set_clauses.join(", "),
        id_ident,
    )
}

/// A derive macro that implements the CRUD `create` function for a struct.
///
///
/// # Attributes
///
/// - `#[crud_id]`: This attribute must be placed on exactly one field within the struct.
///   This field will be automatically initialized by the `create` function.
///
/// # Panics (Compile-time Errors)
///
/// This macro will cause a compile-time error if:
/// - It is applied to an enum or union (only structs are supported).
/// - The struct is not annotated with `#[crud_table("table_name")]`.
/// - No field is annotated with `#[crud_id]`.
/// - More than one field is annotated with `#[crud_id]`.
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

    let query = build_create_query(&id_ident, &table_name, &column_idents);

    quote! {
        // Implement the `create` function for the struct.
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Creates a new entry in the database.
            ///
            /// The `#[crud_table("table_name")]` attribute specifies the database table to insert into.
            /// The field annotated with `#[crud_id]` is used as the identifier for the table.
            pub async fn create(#(#fn_params),*) -> anyhow::Result<Self> {
                let database_url = std::env::var("DATABASE_URL")?;
                let pool = sqlx::SqlitePool::connect(&database_url).await?;

                let item = sqlx::query_as!(#struct_name, #query, #(#column_idents),*,)
                    .fetch_one(&pool)
                    .await?;

                Ok(item)
            }
        }
    }
    .into()
}

#[proc_macro_derive(Read, attributes(crud_id))]
pub fn read_derive(input: TokenStream) -> TokenStream {
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
            /// Reads an entry from the database by its ID.
            ///
            /// The `#[crud_table("table_name")]` attribute specifies the database table to read from.
            /// The field annotated with `#[crud_id]` is used as the identifier for the table.
            pub async fn read(id: i64) -> anyhow::Result<Self> {
                let database_url = std::env::var("DATABASE_URL")?;
                let pool = sqlx::SqlitePool::connect(&database_url).await?;

                let item = sqlx::query_as!(#struct_name, #query, id)
                    .fetch_one(&pool)
                    .await?;

                Ok(item)
            }
        }
    }
    .into()
}

#[proc_macro_derive(ReadAll, attributes(crud_id))]
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
            pub async fn read_all() -> anyhow::Result<Vec<Self>> {
                let database_url = std::env::var("DATABASE_URL")?;
                let pool = sqlx::SqlitePool::connect(&database_url).await?;

                let items: Vec<#struct_name> = sqlx::query_as!(#struct_name, #query)
                    .fetch_all(&pool)
                    .await?;

                Ok(items)
            }
        }
    }
    .into()
}

#[proc_macro_derive(Update, attributes(crud_id))]
pub fn update_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = parse_struct_fields!(input);

    let table_name = parse_table_attribute!(input);
    let id_ident = parse_id_attribute!(fields);
    let (column_idents, _) = parse_column_fields!(fields);

    let query = build_update_query(&id_ident, &table_name, &column_idents);

    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Writes the updated fields of the struct to the database.
            ///
            /// The `#[crud_table("table_name")]` attribute specifies the database table to update.
            /// The field annotated with `#[crud_id]` is used as the identifier for the table.
            pub async fn update(&self) -> anyhow::Result<()> {
                let database_url = std::env::var("DATABASE_URL")?;
                let pool = sqlx::SqlitePool::connect(&database_url).await?;

                sqlx::query!(#query, self.#id_ident, #(self.#column_idents),*)
                    .fetch_all(&pool)
                    .await?;

                Ok(())
            }
        }
    }
    .into()
}

#[proc_macro_derive(Delete, attributes(crud_id))]
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
            pub async fn delete(id: i64) -> anyhow::Result<()> {
                let database_url = std::env::var("DATABASE_URL")?;
                let pool = sqlx::SqlitePool::connect(&database_url).await?;

                sqlx::query!(#query, id)
                    .execute(&pool)
                    .await?;

                Ok(())
            }
        }
    }
    .into()
}
