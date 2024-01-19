use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::types::Buffer;

pub fn encode<T: Serialize>(data: T) -> Buffer {
    Buffer(bincode::serialize(&data).unwrap())
}

pub fn decode<T: DeserializeOwned>(data: &Buffer) -> Result<T, bincode::Error> {
    bincode::deserialize(&data.0)
}