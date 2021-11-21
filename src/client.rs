use std::io::{Read, Write};

pub fn connect<A: std::net::ToSocketAddrs>(address: A) -> Result<(), i32> {
    let mut stream = match std::net::TcpStream::connect(address) {
        Ok(listener) => listener,
        Err(error) => { eprintln!("{}", error); return Err(1) }
    };

    eprintln!("connected to {}", stream.peer_addr().unwrap());

    stream.write(&[0u8, 1] );

    let buffer = &mut [0u8; 4];
    match stream.read_exact(buffer) {
        Ok(()) => {
            eprintln!("read");
            let address = std::net::IpAddr::from(*buffer);
            eprintln!("{}", address.to_string())
        }
        Err(error) => { eprintln!("{}", error); return Err(1) }
    };

    return Ok(());
}