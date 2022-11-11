use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::{self, Error, ErrorKind};

#[cfg(test)]
pub mod test;

pub mod tcp;
pub mod udp;

fn serialize<T: Serialize>(src: &T) -> io::Result<Vec<u8>> {
    bincode::serialize(src).map_err(|_| Error::new(ErrorKind::InvalidData, "can't serialize"))
}

fn deserialize<T: DeserializeOwned>(raw: &[u8]) -> io::Result<T> {
    bincode::deserialize(raw).map_err(|_| Error::new(ErrorKind::InvalidData, "can't deserialize"))
}
