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

pub struct AppConfig<'a> {
    request_url: &'a str,
    output_dir: &'a str,
    duration: i32,
}

impl<'a> AppConfig {
    pub fn new(ip:&'a str,port:&'a str,duration:&'a str,output_dir:&'a str) -> Self {
        let request_url = format!("http://{}:{}/rowcover",ip,port);
        let duration = Duration::from_secs(output_dir.parse().)
    }

    fn get
}