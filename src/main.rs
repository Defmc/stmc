use serde::{Deserialize, Serialize};
use std::{
    io::{BufReader, BufWriter},
    net::{TcpListener, TcpStream},
    thread,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DbAccessLog {
    Name(String),
    Id(u128),
    Close,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = TcpListener::bind(("127.0.0.1", 8080)).unwrap();
    let host = thread::spawn(|| host(conn));
    let client1 = thread::spawn(client);
    client1.join().unwrap();
    host.join().unwrap();
    Ok(())
}

fn host(host: TcpListener) {
    for stream in host.incoming().flatten() {
        let mut stream = BufReader::new(stream);
        let mut buf: Vec<u8> = Vec::with_capacity(std::mem::size_of::<DbAccessLog>());
        loop {
            let msg: DbAccessLog = stmc::read_buf(&mut stream, &mut buf).unwrap();
            // println!("{stream:?}: {msg:?}");
            if msg == DbAccessLog::Close {
                return;
            }
        }
    }
}

fn client() {
    let stream = TcpStream::connect(("127.0.0.1", 8080)).unwrap();
    let mut stream = BufWriter::new(stream);
    for _ in 0..500_000 {
        stmc::send(DbAccessLog::Name("Joe".to_string()), &mut stream).unwrap();
        stmc::send(DbAccessLog::Id(177013), &mut stream).unwrap();
        stmc::send(DbAccessLog::Name("Fynn".to_string()), &mut stream).unwrap();
    }
    stmc::send(DbAccessLog::Close, &mut stream).unwrap();
}
