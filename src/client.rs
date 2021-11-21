use std::io::{Read, Write};
use std::net::TcpStream;

pub fn connect<A: std::net::ToSocketAddrs>(address: A) -> Result<(), i32> {
    let mut stream = match std::net::TcpStream::connect(address) {
        Ok(listener) => listener,
        Err(error) => { eprintln!("{}", error); return Err(1) }
    };

    eprintln!("connected to {}", stream.peer_addr().unwrap());

    match stream.write(&[0u8, 1] ) {
        Ok(_) => {}
        Err(error) => { eprintln!("error writing\n{}", error); return Err(1) }
    };
    match read(stream) {
        Ok(_) => {}
        Err(error) => { eprintln!("error reading\n{}", error); return Err(1) }
    };


    return Ok(());
}

fn read(mut stream: TcpStream) -> std::io::Result<()> {
    let address_type = &mut [0u8, 1];
    stream.read_exact(address_type)?;
    match address_type[0] {
        0 => {
            let buffer = &mut [0u8; 4];
            stream.read_exact(buffer)?;
            let address = std::net::IpAddr::from(*buffer);
            println!("{}", address.to_string())
        }
        1 => {
            let buffer = &mut [0u8; 16];
            stream.read_exact(buffer)?;
            let address = std::net::IpAddr::from(*buffer);
            println!("{}", address.to_string())
        }
        _ => {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "unknown address type"))
        }
    }
    Ok(())
}