#[macro_use]
extern crate log;

use std::process::exit;

use clap::{App, Arg};
use log::{error, warn};
use simplelog::{CombinedLogger, Config, TermLogger, LevelFilter};

use rawcover_collector::{ArgumentsParseError, Collector};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    init_log();

    let syzkaller_rc_collector = match build_config() {
        Ok(app_config) => app_config,
        Err(error) => {
            error!("Error happened while parsing input data:\n{}", error);
            exit(exitcode::DATAERR)
        }
    };

    ctrlc::set_handler(move || {
        warn!("Raw cover data may be lost");
        exit(exitcode::OK);
    }).unwrap_or_else(|error| {
        error!("{}", error);
        exit(exitcode::SOFTWARE);
    });

    syzkaller_rc_collector.run().unwrap_or_else(|error| {
        error!("Error happened while collecting raw cover:\n{}", &error);
        exit(1); // just indicating program exit failed
    });
}

fn init_log() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
        ]
    ).unwrap();
}

fn build_config() -> Result<Collector, ArgumentsParseError> {
    let matches = App::new("rawcover-collector")
        .version(VERSION)
        .author("Sam")
        .about("Syzkaller raw cover collector via http request.")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .takes_value(true).required(true)
            .default_value("http://127.0.0.1:56741/rawcover")
            .help("url of Syzkaller http server"))
        .arg(Arg::with_name("output_dir")
            .short("o")
            .long("output_dir")
            .takes_value(true)
            .default_value(".")
            .help("Output dir for raw cover data"))
        .arg(Arg::with_name("duration")
            .short("d")
            .long("duration")
            .takes_value(true)
            .default_value("15")
            .help("Raw cover collecting duration / second"))
        .get_matches();
    let url = matches.value_of("url").unwrap();// safe here
    let output_dir = matches.value_of("output_dir").unwrap();
    let duration = matches.value_of("duration").unwrap();
    Collector::new(url, duration, output_dir)
}
