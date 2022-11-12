Just a simple TCP/UDP Message Communication wrapper.
Uses `serde` to data serialization in `bincode` format. Requires the type to derive `serde::Serialize` for sending and `serde:Deserialize` for receiving messages.

Consider the following enum:
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Chat {
    Connect,
    Msg(String),
    Close,
}
```

Receiving messages:
```rust
use std::{io, net::TcpListener};
use stmc::tcp;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 8080))?;
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
```

Sending messages:
```rust
use serde::{Deserialize, Serialize};
use std::{io, net::TcpStream};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect(("127.0.0.1", 8080))?;
    tcp::send(Chat::Connect, &mut stream)?;
    tcp::send(Chat::Msg("hi".into()), &mut stream)?;
    tcp::send(Chat::Close, &mut stream)?;
    Ok(())
}
```

To use UDP protocol, just change from `tcp` to `udp`.
