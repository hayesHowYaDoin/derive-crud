pub enum CRUDError {
    NotFound(String),
    AlreadyExists(String),
    InvalidInput(String),
    Unauthorized(String),
    InternalError(String),
}
