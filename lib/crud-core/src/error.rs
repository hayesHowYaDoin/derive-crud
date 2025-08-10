/// Errors that can occur in CRUD operations.
#[derive(Debug)]
pub enum CRUDError {
    /// Occurs when the DATABASE_URL environment variable is not set.
    EnvNotSet,
    /// Occurs when the database connection fails.
    ConnectionError(String),
    /// Occurs when the query to the database fails.
    QueryError(String),
}
