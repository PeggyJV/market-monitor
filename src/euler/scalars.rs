use serde::{Deserialize, Serialize};

/// Used as a placeholder for the Bytes GraphQL scalar type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bytes(pub String);

/// Used as a placeholder for the BigInt GraphQL scalar type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BigInt(pub String);
