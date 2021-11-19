use std::io::Read;

pub fn listen<A: std::net::ToSocketAddrs>(address: A) -> Result<(), i32> {

    let listener = std::net::TcpListener::bind(address).unwrap();
    eprintln!("Server started!");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut buffer = std::io::BufReader::new(stream);
                let b = &mut [];
                match buffer.read_exact(b) {
                    Ok(_todo) => eprintln!("ok"),
                    Err(e) => { eprintln!("read failed {}", e); return Err(1) }
                }
            },
            Err(e) => { eprintln!("Connection failed: {}", e); return Err(1) }
        }
    }
    return Ok(());
}