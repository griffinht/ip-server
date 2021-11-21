use std::io::{Read, Write};

pub fn connect<A: std::net::ToSocketAddrs>(address: A) -> std::io::Result<()> {
    let mut stream = std::net::TcpStream::connect(address)?;

    eprintln!("connected to {}", stream.peer_addr().unwrap());

    stream.write(&[0u8, 1] )?;

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