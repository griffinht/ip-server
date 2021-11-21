pub fn matches() -> std::result::Result<getopts::Matches, i32> {
    let arguments: Vec<String>;

    arguments = std::env::args().collect();

    let mut options = getopts::Options::new();

    options.optflag("h", "help", "print help");
    options.optflag("v", "version", "print version");
    options.optopt("c", "client", concat!("run as client, connect to address (default port :", crate::default_port!(), ")"), "<address>");
    options.optopt("s", "server", concat!("run as server, bind to address (default ", crate::default_bind_address!(), ":", crate::default_port!(), ")"), "<address>");

    let matches = match options.parse(&arguments[1..]) {
        Ok(matches) => matches,
        Err(fail) => { eprintln!("{}", fail); return Err(1); }
    };
    if matches.opt_present("h") {
        eprint!("{}", options.usage_with_format(|opts| {
            format!(
                concat!("Usage: ", crate::name!(), " [options...]\n{}\n"),
                opts.collect::<Vec<String>>().join("\n")
            )
        }));
        return Err(0)
    }
    if matches.opt_present("v") {
        eprintln!(concat!(crate::name!(), " version ", crate::version!()));
        return Err(0)
    }

    Ok(matches)
}