use std::io::{Write};

pub fn listen<A: std::net::ToSocketAddrs>(address: A) -> Result<(), i32> {

    let listener = match std::net::TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(error) => { eprintln!("{}", error); return Err(1) }
    };
    eprintln!("listening on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut headers: Vec<u8> = Vec::new();
                let mut body: Vec<u8> = Vec::new();

                let peer_address = stream.peer_addr().unwrap();
                eprintln!("{}", peer_address);
                match peer_address.ip() {
                    std::net::IpAddr::V4(ip) => {
                        body.extend_from_slice(ip.to_string().as_bytes())
                    },
                    std::net::IpAddr::V6(ip) => {
                        body.extend_from_slice(ip.to_string().as_bytes())
                    }
                }
/*                match peer_address.ip() {
                    std::net::IpAddr::V4(ip) => {
                        body.extend_from_slice(&ip.octets())
                    },
                    std::net::IpAddr::V6(ip) => {
                        body.extend_from_slice(&ip.octets())
                    }
                }*/

                headers.extend_from_slice("HTTP/1.1 200 OK\r\nContent-Type: text/plain;\r\nContent-Length: ".as_bytes());
                headers.extend_from_slice(body.len().to_string().as_bytes());
                headers.extend_from_slice("\r\n\r\n".as_bytes());

                match stream.write(&headers) {
                    Ok(b) => { eprintln!("wrote {}", b)}
                    Err(error) => { eprintln!("error {}", error) }
                }
                match stream.write(&body) {
                    Ok(b) => { eprintln!("wrote {}", b)}
                    Err(error) => { eprintln!("error {}", error) }
                }
            },
            Err(e) => { eprintln!("Connection failed: {}", e); return Err(1) }
        }
    }
    return Ok(());
}