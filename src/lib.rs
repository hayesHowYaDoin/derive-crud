//! # derive-crud
//!
//! `derive-crud` is a set of derive macros which automatically implement CRUD
//! access functions. Under the hood, it uses the SQLx crate's `query!` macro
//! to generate SQL queries at compile time, ensuring type safety and performance.

pub use crud_core::error::CRUDError;
pub use crud_macro::{Create, Delete, Read, ReadAll, ReadOne, Update};
