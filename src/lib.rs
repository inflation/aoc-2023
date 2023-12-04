#[derive(Debug)]
pub struct AdHocError {
    pub message: String,
}

impl AdHocError {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for AdHocError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AdHocError {}
