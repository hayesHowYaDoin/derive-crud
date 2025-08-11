/// Error type returned by generated CRUD functions.
#[derive(Debug)]
pub struct CRUDError(String);

impl CRUDError {
    pub fn new(message: impl Into<String>) -> Self {
        CRUDError(message.into())
    }
}

impl std::fmt::Display for CRUDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CRUDError {}
