use std::io::{Read, Write};
use std::net::TcpStream;

pub fn listen<A: std::net::ToSocketAddrs>(address: A) -> Result<(), i32> {

    let listener = match std::net::TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(error) => { eprintln!("{}", error); return Err(1) }
    };
    eprintln!("listening on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let protocol = &mut [0u8; 1];
                match stream.read_exact(protocol) {
                    Ok(_) => {}
                    Err(_) => {}
                }
                match match protocol[0] {
                    0 => { write_raw(stream) }
                    _ => { write_http(stream) }
                } {
                    Ok(()) => {}
                    Err(error) => { eprintln!("error {}", error); return Err(1) }
                }

            },
            Err(e) => { eprintln!("Connection failed: {}", e); return Err(1) }
        }
    }
    return Ok(());
}

fn write_raw(mut stream: TcpStream) -> std::io::Result<()> {
    let peer_address = stream.peer_addr().unwrap();
    eprintln!("{}", peer_address);
    match peer_address.ip() {
        std::net::IpAddr::V4(ip) => {
            eprintln!("wrote {}", stream.write(&ip.octets())?);
        },
        std::net::IpAddr::V6(ip) => {
            eprintln!("wrote {}", stream.write(&ip.octets())?);
        }
    }
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