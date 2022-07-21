use anyhow::Result;
use clap::{AppSettings, Arg, Command};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Command::new("Echo Server")
        .version("0.1.0")
        .author("Rain Liu <y@liu.mx>")
        .about("An example of echo server that receives a string and writes it straight back.")
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand_negates_reqs(true)
        .arg(
            Arg::new("FULLHELP")
                .help("Prints more detailed help information")
                .long("fullhelp"),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .short('d')
                .help("Prints debug log information"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .default_value("8080")
                .help("Echo server port"),
        );

    let matches = app.clone().get_matches();

    if matches.is_present("FULLHELP") {
        app.print_long_help().unwrap();
        std::process::exit(0);
    }

    let debug = matches.is_present("debug");
    if debug {
        env_logger::Builder::new()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{}:{} [{}] {} - {}",
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    record.level(),
                    chrono::Local::now().format("%H:%M:%S.%6f"),
                    record.args()
                )
            })
            .filter(None, log::LevelFilter::Trace)
            .init();
    }

    Ok(())
}
