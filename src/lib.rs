use std::io::{self, Read, Write};

use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn send<T>(msg: T, stream: &mut impl Write) -> io::Result<()>
where
    T: Serialize,
{
    let raw = bincode::serialize(&msg).unwrap();
    let len: u32 = raw.len().try_into().unwrap();
    stream.write_all(&len.to_le_bytes()).unwrap();
    stream.write_all(&raw).unwrap();
    Ok(())
}

pub fn read<T>(stream: &mut impl Read) -> Result<T, ()>
where
    T: DeserializeOwned,
{
    let mut buf_size = [0u8; 4];
    stream.read_exact(&mut buf_size).unwrap();
    let size = u32::from_le_bytes(buf_size);
    let mut buff = vec![0u8; size as usize];
    stream.read_exact(&mut buff).unwrap();
    bincode::deserialize(&buff).map_err(|_| ())
}

pub fn read_buf<T>(stream: &mut impl Read, buf: &mut Vec<u8>) -> Result<T, ()>
where
    T: DeserializeOwned,
{
    let mut buf_size = [0u8; 4];
    stream.read_exact(&mut buf_size).unwrap();
    let size = u32::from_le_bytes(buf_size);
    buf.resize(size as usize, 0u8);
    stream.read_exact(buf).unwrap();
    bincode::deserialize(&buf).map_err(|_| ())
}
