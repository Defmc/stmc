use crate as stmc;
use serde::{Deserialize, Serialize};
use std::{
    io,
    net::{TcpListener, TcpStream},
    thread,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Chat {
    Connect,
    Msg(String),
    Close,
}

#[test]
pub fn readme() {
    fn host(listener: TcpListener) -> io::Result<()> {
        'server: for mut stream in listener.incoming().flatten() {
            loop {
                let msg: Chat = stmc::read(&mut stream)?;
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
        stmc::send(Chat::Connect, &mut stream)?;
        stmc::send(Chat::Msg("hi".into()), &mut stream)?;
        stmc::send(Chat::Close, &mut stream)?;
        Ok(())
    }

    let listener = TcpListener::bind(("127.0.0.1", 8080)).unwrap();
    let th_host = thread::spawn(move || host(listener));
    let th_client = thread::spawn(client);

    th_client.join().unwrap().unwrap();
    th_host.join().unwrap().unwrap();
}
