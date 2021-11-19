use std::io::{Read, Write};

pub fn listen<A: std::net::ToSocketAddrs>(address: A) -> Result<(), i32> {

    let listener = match std::net::TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(error) => { eprintln!("{}", error); return Err(1) }
    };
    eprintln!("listening on {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let peer_address = stream.peer_addr().unwrap();
                eprintln!("{}", peer_address);
                match stream.write(match peer_address.ip() {
                    std::net::IpAddr::V4(ip) => stream.write(&*ip.octets()),
                    std::net::IpAddr::V6(ip) => stream.write(&*ip.octets())
                }) {
                    Ok(b) => { eprintln!("wrote {}", b)}
                    Err(error) => { eprintln!("error {}", error) }
                }
            },
            Err(e) => { eprintln!("Connection failed: {}", e); return Err(1) }
        }
    }
    return Ok(());
}