#[derive(Debug)]
pub struct CRUDError(pub String);

impl std::fmt::Display for CRUDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CRUDError {}
