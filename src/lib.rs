use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::{self, Error, ErrorKind};

pub type MsgSize = u32;
pub const SIZE_BYTES: usize = std::mem::size_of::<MsgSize>();

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

fn from_bytes(raw: &[u8; SIZE_BYTES]) -> MsgSize {
    if cfg!(features = "big_endian") {
        MsgSize::from_be_bytes(*raw)
    } else {
        MsgSize::from_le_bytes(*raw)
    }
}

fn to_bytes(src: MsgSize) -> [u8; SIZE_BYTES] {
    if cfg!(features = "big_endian") {
        src.to_be_bytes()
    } else {
        src.to_le_bytes()
    }
}
