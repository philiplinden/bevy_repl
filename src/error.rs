// In error.rs
#[derive(Debug)]
pub enum ReplError {
    CommandNotFound(String),
    ClapError(clap::Error),
    ExecutionError(Box<dyn std::error::Error + Send + Sync>),
    WorldAccessError(String),
}

impl std::fmt::Display for ReplError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReplError::CommandNotFound(cmd) => write!(f, "Command '{}' not found", cmd),
            ReplError::ClapError(e) => write!(f, "{}", e),
            ReplError::ExecutionError(e) => write!(f, "Execution error: {}", e),
            ReplError::WorldAccessError(msg) => write!(f, "World access error: {}", msg),
        }
    }
}
