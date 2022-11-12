use crate::{MsgSize, SIZE_BYTES};
use serde::{de::DeserializeOwned, Serialize};
use std::io::{self, Error, ErrorKind};
use std::net::UdpSocket;

pub fn send<T>(msg: &T, sock: &mut UdpSocket) -> io::Result<()>
where
    T: Serialize,
{
    let raw = crate::serialize(msg)?;
    let len: MsgSize = raw
        .len()
        .try_into()
        .map_err(|_| Error::new(ErrorKind::OutOfMemory, "size exceds the MsgSize limit"))?;
    let mut pkg = len.to_le_bytes().to_vec();
    pkg.extend(raw);
    sock.send(dbg!(&pkg))?;
    Ok(())
}

pub fn read<T>(sock: &mut UdpSocket) -> io::Result<T>
where
    T: DeserializeOwned,
{
    read_buf(sock, &mut Vec::new())
}

pub fn read_buf<T>(sock: &mut UdpSocket, buf: &mut Vec<u8>) -> io::Result<T>
where
    T: DeserializeOwned,
{
    let mut buf_size = [0u8; SIZE_BYTES];
    sock.peek(&mut buf_size)?;
    let len = MsgSize::from_le_bytes(dbg!(buf_size));
    buf.resize(len as usize + SIZE_BYTES, 0u8);
    sock.recv(buf)?;
    crate::deserialize(&buf[SIZE_BYTES..])
}
