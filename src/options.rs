pub fn matches() -> std::result::Result<getopts::Matches, ()> {
    let arguments: Vec<String>;

    arguments = std::env::args().collect();

    let mut options = getopts::Options::new();

    options.optflag("h", "help", "print help");
    options.optflag("v", "version", "print version");
    options.optopt("c", "client", "run as client, connect to address (default port :9999)", "<address>");
    options.optopt("s", "server", "run as server, bind to address (default 0.0.0.0:9999)", "<address>");

    let matches = match options.parse(&arguments[1..]) {
        Ok(matches) => matches,
        Err(fail) => { eprintln!("{}", fail); return Err(()) }
    };
    if matches.opt_present("h") {
        eprint!("{}", options.usage_with_format(|opts| {
            format!(
                "Usage: {} [options...] <command>\n{}\n",
                crate::NAME,
                opts.collect::<Vec<String>>().join("\n")
            )
        }));
        return Err(())
    }
    if matches.opt_present("v") {
        eprintln!("{} version {}", crate::NAME, crate::VERSION);
        return Err(())
    }

    Ok(matches)
}