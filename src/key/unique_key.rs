use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FinternetUID([u8; 32]);
