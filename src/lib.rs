//! # derive-crud
//!
//! `derive-crud` is a set of derive macros which automatically implement CRUD
//! access functions. Under the hood, it uses the SQLx crate's `query!` macro
//! to generate SQL queries at compile time, ensuring type safety and performance.

pub use crud_core::error::CRUDError;
pub use crud_macro::{Create, Delete, Read, Update};

// Re-export to ensure crate dependencies are available at compilation.
pub use async_stream;
pub use futures_core;
pub use futures_util;
pub use sqlx;
