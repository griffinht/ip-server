use std::io::{Read, Write};
use std::net::TcpStream;

pub fn listen<A: std::net::ToSocketAddrs>(address: A) -> Result<(), i32> {
    match _listen(address) {
        Ok(()) => Ok(()),
        Err(error) => { eprintln!("error :(\n{}", error); Err(1) }
    }
}

fn _listen<A: std::net::ToSocketAddrs>(address: A) -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind(address)?;
    eprintln!("listening on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        let mut stream = stream?;
        let protocol = &mut [0u8; 1];
        stream.read_exact(protocol)?;
        match protocol[0] {
            0 => { write_raw(stream) }
            71 => { write_http(stream) } //71 represents ASCII letter G which is sent from an HTTP GET request
            _ => { return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "unknown protocol")); }
        }?;
    }
    Ok(())
}

fn write_raw(mut stream: TcpStream) -> std::io::Result<()> {
    let mut response: Vec<u8> = Vec::new();
    let peer_address = stream.peer_addr().unwrap();
    eprintln!("{}", peer_address);
    match peer_address.ip() {
        std::net::IpAddr::V4(ip) => {
            response.extend_from_slice(&[0u8, 0] );
            response.extend_from_slice(&ip.octets())
        },
        std::net::IpAddr::V6(ip) => {
            response.extend_from_slice(&[0u8, 1] );
            response.extend_from_slice(&ip.octets())
        }
    };
    eprintln!("wrote {}", stream.write(&response)?);
    Ok(())
}

fn write_http(mut stream: TcpStream) -> std::io::Result<()> {
    let mut headers: Vec<u8> = Vec::new();
    let mut body: Vec<u8> = Vec::new();

    let peer_address = stream.peer_addr().unwrap();
    eprintln!("{}", peer_address);
    match peer_address.ip() {
        std::net::IpAddr::V4(ip) => {
            body.extend_from_slice(ip.to_string().as_bytes());
        },
        std::net::IpAddr::V6(ip) => {
            body.extend_from_slice(ip.to_string().as_bytes());
        }
    }

    headers.extend_from_slice("HTTP/1.1 200 OK\r\nContent-Type: text/plain;\r\nContent-Length: ".as_bytes());
    headers.extend_from_slice(body.len().to_string().as_bytes());
    headers.extend_from_slice("\r\n\r\n".as_bytes());

    eprintln!("wrote {}", stream.write(&headers)?);
    eprintln!("wrote {}", stream.write(&body)?);

    Ok(())
}