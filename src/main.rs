mod options;

use std::io::Read;

pub const NAME: &str = "rust-ip";
pub const VERSION: &str = "0.1.0";

fn main() {
    match _main() {
        Ok(()) => {},
        Err(code) => std::process::exit(code)
    }
}

fn _main() -> Result<(), i32> {

    let _matches = match options::matches() {
        Ok(matches) => matches,
        Err(()) => return Err(1)
    };

    let listener = std::net::TcpListener::bind("0.0.0.0:8000").unwrap();
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

    Ok(())
}
