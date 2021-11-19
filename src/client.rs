pub fn connect<A: std::net::ToSocketAddrs>(address: A)  -> Result<(), i32> {
    let stream = match std::net::TcpStream::connect(address) {
        Ok(listener) => listener,
        Err(error) => { eprintln!("{}", error); return Err(1) }
    };
    eprintln!("connected to {}", stream.peer_addr().unwrap());

    return Ok(());
}