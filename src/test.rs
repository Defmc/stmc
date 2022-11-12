use crate::{tcp, udp};
use serde::{Deserialize, Serialize};
use std::{
    io,
    net::{TcpListener, TcpStream, UdpSocket},
    thread,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Chat {
    Connect,
    Msg(String),
    Close,
}

#[test]
pub fn tcp_readme() {
    fn host(listener: TcpListener) -> io::Result<()> {
        'server: for mut stream in listener.incoming().flatten() {
            loop {
                let msg: Chat = tcp::read(&mut stream)?;
                match msg {
                    Chat::Connect => println!("new user connect: {}", stream.ttl()?),
                    Chat::Msg(txt) => println!("new message from {}: {txt}", stream.ttl()?),
                    Chat::Close => break 'server,
                }
            }
        }
        Ok(())
    }

    fn client() -> io::Result<()> {
        let mut stream = TcpStream::connect(("127.0.0.1", 8080))?;
        tcp::send(&Chat::Connect, &mut stream)?;
        tcp::send(&Chat::Msg("hi".into()), &mut stream)?;
        tcp::send(&Chat::Close, &mut stream)?;
        Ok(())
    }

    let listener = TcpListener::bind(("127.0.0.1", 8080)).unwrap();
    let th_host = thread::spawn(move || host(listener));
    let th_client = thread::spawn(client);

    th_client.join().unwrap().unwrap();
    th_host.join().unwrap().unwrap();
}

#[test]
pub fn udp_readme() {
    fn host(mut listener: UdpSocket) -> io::Result<()> {
        loop {
            let msg: Chat = udp::read(&mut listener)?;
            match msg {
                Chat::Connect => println!("new user connect: {}", listener.ttl()?),
                Chat::Msg(txt) => println!("new message from {}: {txt}", listener.ttl()?),
                Chat::Close => break,
            }
        }
        Ok(())
    }

    fn client() -> io::Result<()> {
        let mut stream = UdpSocket::bind(("127.0.0.1", 8081))?;
        stream.connect(("127.0.0.1", 8080))?;
        udp::send(&Chat::Connect, &mut stream)?;
        udp::send(&Chat::Msg("hi".into()), &mut stream)?;
        udp::send(&Chat::Close, &mut stream)?;
        Ok(())
    }

    let listener = UdpSocket::bind(("127.0.0.1", 8080)).unwrap();
    let th_host = thread::spawn(move || host(listener));
    let th_client = thread::spawn(client);

    th_client.join().unwrap().unwrap();
    th_host.join().unwrap().unwrap();
}
