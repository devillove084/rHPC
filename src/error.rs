use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum GraphComputationError {
    #[snafu(display("Invalid node reference: {}", node))]
    InvalidNode { node: usize },

    #[snafu(display("Edge already exists between {} and {}", node1, node2))]
    EdgeExists { node1: usize, node2: usize },

    #[snafu(display("Cycle detected in the graph"))]
    CycleDetected,
}

#[derive(Debug, Snafu)]
pub enum TensorComputationError {
    #[snafu(display("Shape mismatch: expected {:?}, but got {:?}", expected, found))]
    ShapeMismatch {
        expected: Vec<usize>,
        found: Vec<usize>,
    },

    #[snafu(display("Arithmetic operation failed: {}", details))]
    ArithmeticError { details: String },
}

#[derive(Debug, Snafu)]
pub enum NonNumericComputationError {
    #[snafu(display("Failed to parse input: {}", details))]
    ParseError { details: String },

    #[snafu(display("Pattern matching error: {}", details))]
    PatternMatchError { details: String },

    #[snafu(display("Query execution error: {}", details))]
    QueryExecutionError { details: String },

    #[snafu(display("Unknown error occurred"))]
    UnknownError,
}

#[derive(Debug, Snafu)]
pub enum ComputeError {
    #[snafu(display("Failed to allocate memory for tensor: {}", source))]
    MemoryAllocationError { source: std::io::Error },

    #[snafu(display("Invalid operation on tensor: {}", message))]
    InvalidOperation { message: String },

    #[snafu(display("Device execution error: {}", source))]
    DeviceError { source: std::io::Error },

    #[snafu(display("General computation error: {}", message))]
    GeneralError { message: String },
}

pub type Result<T, E = ComputeError> = std::result::Result<T, E>;
