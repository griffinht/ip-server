use std::io::Read;

fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:8000")?;
    eprintln!("Server started!");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut buffer = std::io::BufReader::new(stream);
                let b = &mut [];
                match buffer.read_exact(b, 4) {
                    Ok(size) => eprintln!("ok"),
                    Err(e) => eprintln!("read failed {}", e)
                }
            },
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
