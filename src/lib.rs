use std::io::{self, Read, Write};

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::{Error, ErrorKind};

/// Send a message through a stream
/// # Errors
/// Can fail in the data serialization
/// Fail when struct serialization exceds `u32::MAX`
/// Depends from writer to send the serialized data
pub fn send<T>(msg: T, stream: &mut impl Write) -> io::Result<()>
where
    T: Serialize,
{
    let raw = bincode::serialize(&msg)
        .map_err(|_| Error::new(ErrorKind::InvalidData, "can't serialize"))?;
    let len: u32 = raw
        .len()
        .try_into()
        .map_err(|_| Error::new(ErrorKind::OutOfMemory, "size exceds the u32 limit"))?;
    stream.write_all(&len.to_le_bytes())?;
    stream.write_all(&raw)
}

/// Read a message from a reader
/// # Errors
/// Fail when receives a package with a different size from `std::mem::size_of<T>() + 4`
/// Can fail in the deserialization process
pub fn read<T>(stream: &mut impl Read) -> io::Result<T>
where
    T: DeserializeOwned,
{
    let mut buf_size = [0u8; 4];
    stream.read_exact(&mut buf_size)?;
    let size = u32::from_le_bytes(buf_size);
    let mut buff = vec![0u8; size as usize];
    stream.read_exact(&mut buff)?;
    bincode::deserialize(&buff).map_err(|_| Error::new(ErrorKind::InvalidData, "can't deserialize"))
}

/// Read a message using a pre-allocated buffer
/// # Errors
/// Same of `stmc::read`
pub fn read_buf<T>(stream: &mut impl Read, buf: &mut Vec<u8>) -> io::Result<T>
where
    T: DeserializeOwned,
{
    let mut buf_size = [0u8; 4];
    stream.read_exact(&mut buf_size)?;
    let size = u32::from_le_bytes(buf_size);
    buf.resize(size as usize, 0u8);
    stream.read_exact(buf)?;
    bincode::deserialize(buf).map_err(|_| Error::new(ErrorKind::InvalidData, "can't deserialize"))
}