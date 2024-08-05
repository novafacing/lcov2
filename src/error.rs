use petgraph::prelude::NodeIndex;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
/// An error raised by the `lcov` library
pub enum Error {
    #[error("Node not found for path {path:?}")]
    /// Raised when a node is not found for a given path
    NodeNotFound { path: PathBuf },
    #[error("Weight not found for node index {index:?}")]
    /// Raised when a weight is not found for a given node index
    WeightNotFound { index: NodeIndex },
    #[error("No summary info for node")]
    /// Raised when no summary info is found for a node
    NoSummaryInfo,
    #[error(transparent)]
    /// A wrapped std::io::Error
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    /// A wrapped std::fmt::Error
    FmtError(#[from] std::fmt::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
