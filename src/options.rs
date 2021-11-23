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
    #[test]
    fn matches() -> std::result::Result<(), i32> {
        macro_rules! vec_of_strings {
            ($str:expr) => ({
                $str.iter().map(|s:&&str | s.to_string()).collect()
            });
        }
        crate::options::matches(vec_of_strings!([]))?;
        crate::options::matches(vec_of_strings!(["asd"]))?;
        crate::options::matches(vec_of_strings!(["asd", "sdf"]))?;
        Ok(())
    }
}