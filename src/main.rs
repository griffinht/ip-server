mod options;
mod server;
mod client;

#[macro_export]
macro_rules! name {
    () => ("rust-ip")
}
#[macro_export]
macro_rules! version {
    () => ("0.1.0")
}
#[macro_export]
macro_rules! default_bind_address {
    () => ("0.0.0.0")
}
#[macro_export]
macro_rules! default_port {
    () => (8000)
}

fn main() {
    match _main() {
        Ok(_) => {},
        Err(code) => std::process::exit(code)
    }
}

fn _main() -> Result<(), i32> {

    let matches = options::matches()?;

    return if matches.opt_present("client") {
        client::connect(matches.opt_get::<String>("client").unwrap().unwrap())
    } else {
        server::listen(
            if matches.opt_present("server") {
                matches.opt_get::<String>("server").unwrap().ok_or(1)?
            } else {
                concat!(default_bind_address!(), ":", default_port!()).to_string()
            }
        )
    }
}
