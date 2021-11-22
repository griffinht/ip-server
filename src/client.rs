use std::io::{Read, Write};

pub fn connect<A: std::net::ToSocketAddrs>(address: A) -> std::io::Result<()> {
    let mut stream = std::net::TcpStream::connect(address)?;

    stream.write(&[0u8] )?;//raw protocol

    let address_type = &mut [0u8; 1];
    stream.read_exact(address_type)?;
    let ipaddr: std::net::IpAddr = match address_type[0] {
        0 => { //ipv4
            let buffer = &mut [0u8; 4];
            stream.read_exact(buffer)?;
            std::net::IpAddr::from(*buffer)
        }
        1 => { //ipv6
            let buffer = &mut [0u8; 16];
            stream.read_exact(buffer)?;
            std::net::IpAddr::from(*buffer)
        }
        _ => {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, concat!("bad protocol: unknown address type (is this a ", env!("CARGO_PKG_NAME"), " server?)")))
        }
    };
    println!("{}", ipaddr.to_string());
    Ok(())
}