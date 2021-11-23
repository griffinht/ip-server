pub fn matches(arguments: Vec<String>) -> std::result::Result<getopts::Matches, i32> {

    let mut options = getopts::Options::new();

    options.optflag("h", "help", "print help");
    options.optflag("v", "version", "print version");
    options.optopt("c", "client", "run as client, connect to address", "<address>");
    options.optopt("s", "server", concat!("run as server, bind to address (default ", crate::default_bind_address!(), ":", crate::default_port!(), ")"), "<address>");

    let matches = match options.parse(&arguments[1..]) {
        Ok(matches) => matches,
        Err(fail) => { eprintln!("{}", fail); return Err(1); }
    };
    if matches.opt_present("h") {
        eprint!("{}", options.usage_with_format(|opts| {
            format!(
                concat!("Usage: ", env!("CARGO_PKG_NAME"), " [options...]\n{}\n"),
                opts.collect::<Vec<String>>().join("\n")
            )
        }));
        return Err(0)
    }
    if matches.opt_present("v") {
        eprintln!(concat!(env!("CARGO_PKG_NAME"), " version ", env!("CARGO_PKG_VERSION")));
        return Err(0)
    }

    Ok(matches)
}

mod tests {
    macro_rules! to_string_vec {
        ($str:expr) => ({
            $str.iter().map(|s:&&str | s.to_string()).collect()
        });
    }
    #[test]
    fn matches() -> std::result::Result<(), i32> {
        crate::options::matches(to_string_vec!(["asd"]))?;
        crate::options::matches(to_string_vec!(["asd"]))?;
        crate::options::matches(to_string_vec!(["asd", "sdf"]))?;
        Ok(())
    }
    #[test]
    #[should_panic]
    fn no_program_first_arg() {
        match crate::options::matches(to_string_vec!([])) {
            _ => {}
        };
    }
}