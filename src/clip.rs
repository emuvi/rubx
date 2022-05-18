use clap::{Arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    Command::new("rubx")
    .version(clap::crate_version!())
    .about("Rubx (Rust Basic Toolbox) is a library for command programs that features a toolbox with a series of basic functionalities.")
    .author("Éverton M. Vieira <emuvi@outlook.com.br>")
    .arg(
      Arg::new("verbose")
        .short('v')
        .long("verbose")
        .takes_value(false)
        .required(false)
        .help("Prints verbose information."),
    )
    .arg(
      Arg::new("archive")
        .short('a')
        .long("archive")
        .takes_value(false)
        .required(false)
        .help("Saves the archive log on a file."),
    )
    .arg(
      Arg::new("debug-calls")
        .short('c')
        .long("debug-calls")
        .takes_value(false)
        .required(false)
        .help("If has debug symbols, is debugged the functions calls."),
    )
    .arg(
      Arg::new("debug-reavs")
        .short('r')
        .long("debug-reavs")
        .takes_value(false)
        .required(false)
        .help("If has debug symbols, is debugged the functions returns."),
    )
    .arg(
      Arg::new("debug-steps")
        .short('s')
        .long("debug-steps")
        .takes_value(false)
        .required(false)
        .help("If has debug symbols, is debugged the functions operations."),
    )
    .arg(
      Arg::new("debug-tells")
        .short('t')
        .long("debug-tells")
        .takes_value(false)
        .required(false)
        .help("If has debug symbols, is debugged the functions iterations."),
    ).get_matches()
}
