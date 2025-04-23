use serde::{Deserialize, Serialize};

/// Used as a placeholder for the Bytes GraphQL scalar type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bytes(pub String);
