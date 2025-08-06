/// Errors that can occur in CRUD operations.
pub enum CRUDError {
    /// Indicates that the requested resource was not found.
    NotFound(String),
    /// Indicates that the resource already exists.
    AlreadyExists(String),
    /// Indicates that the input provided is invalid.
    InvalidInput(String),
    /// Indicates that the operation is unauthorized.
    Unauthorized(String),
    /// Indicates that an internal error occurred.
    InternalError(String),
}
