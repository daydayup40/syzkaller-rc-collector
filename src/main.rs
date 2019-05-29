extern crate chrono;
extern crate clap;
extern crate ctrlc;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate simplelog;
extern crate url;


use std::net::{IpAddr, Ipv4Addr};
use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread::{JoinHandle, sleep, spawn};
use std::time::{Duration, Instant};

use clap::{App, Arg, ArgMatches};
use log::LevelFilter;
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use simplelog::{CombinedLogger, Config, TermLogger};
use url::{OpaqueOrigin, Url};

use rawcover_collector::AppConfig;
use rawcover_collector::collector::Requestor;

const RAW_COVER: &'static str = "rawcover";
const HTTP_PREFIX: &'static str = "http://";

fn main() {
    init_log();
    let app_config = build_config();
    let requestor = Requestor {};
    info!("{:?}", requestor.request(app_config.request_url.as_str()));
}

fn init_log() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
        ]
    ).unwrap();
}

fn build_config() -> AppConfig {
    let matches = App::new("RC-Collector")
        .version("0.1.0")
        .author("Sam")
        .about("Syzkaller raw cover collector via http request.")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .takes_value(true).required(true)
            .help("url  of Syzkaller http server"))
        .arg(Arg::with_name("output_dir")
            .short("o")
            .long("output_dir")
            .takes_value(true)
            .default_value(".")
            .help("Output dir,default '.'"))
        .arg(Arg::with_name("duration")
            .short("du")
            .long("duration")
            .takes_value(true)
            .default_value("30")
            .help("Raw cover collecting duration / minute"))
        .get_matches();
    let url = matches.value_of("url").unwrap();// safe here
    let output_dir = matches.value_of("output_dir").unwrap();
    let duration = matches.value_of("duration").unwrap();
    match AppConfig::new(url, duration, output_dir) {
        Ok(app_config) => app_config,
        Err(error) => {
            error!("{}", error);
            exit(exitcode::CONFIG)
        }
    }
}
