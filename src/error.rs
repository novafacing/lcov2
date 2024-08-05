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
    #[error(transparent)]
    /// A wrapped std::num::ParseIntError
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    /// A wrapped infallible
    Infallible(#[from] std::convert::Infallible),
    #[error("Invalid test name record entry {record}")]
    /// An invalid test name record entry
    InvalidTestNameRecordEntry { record: String },
    #[error("Invalid source file record entry {record}")]
    /// An invalid source file record entry
    InvalidSourceFileRecordEntry { record: String },
    /// An invalid version record entry
    #[error("Invalid version record entry {record}")]
    InvalidVersionRecordEntry { record: String },
    #[error("Invalid function record entry {record}")]
    /// An invalid function record entry
    InvalidFunctionRecordEntry { record: String },
    #[error("Invalid function data record entry {record}")]
    /// An invalid function record entry
    InvalidFunctionDataRecordEntry { record: String },
    #[error("Invalid functions found record entry {record}")]
    /// An invalid functions found record entry
    InvalidFunctionsFoundRecordEntry { record: String },
    #[error("Invalid functions hit record entry {record}")]
    /// An invalid functions hit record entry
    InvalidFunctionsHitRecordEntry { record: String },
    #[error("Invalid branch data record entry {record}")]
    /// An invalid branch data record entry
    InvalidBranchDataRecordEntry { record: String },
    #[error("Invalid branches found record entry {record}")]
    /// An invalid branches found record entry
    InvalidBranchesFoundRecordEntry { record: String },
    #[error("Invalid branches hit record entry {record}")]
    /// An invalid branches hit record entry
    InvalidBranchesHitRecordEntry { record: String },
    #[error("Invalid line data record entry {record}")]
    /// An invalid line data record entry
    InvalidLineDataRecordEntry { record: String },
    #[error("Invalid lines found record entry {record}")]
    /// An invalid lines found record entry
    InvalidLinesFoundRecordEntry { record: String },
    #[error("Invalid lines hit record entry {record}")]
    /// An invalid lines hit record entry
    InvalidLinesHitRecordEntry { record: String },
    #[error("Invalid end of record entry {record}")]
    /// An invalid end of record entry
    InvalidEndOfRecordEntry { record: String },
    #[error("Invalid record entry {record}")]
    /// An invalid record entry
    InvalidRecordEntry { record: String },
}

pub type Result<T> = std::result::Result<T, Error>;
