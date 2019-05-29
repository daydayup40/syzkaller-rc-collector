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

const RAW_COVER: &'static str = "rawcover";
const HTTP_PREFIX: &'static str = "http://";

fn main() {
    init_log();
    let (ip, port, output_dir, duration) = build_command_match();
    let mut request_url = build_url(&ip, &port).to_string();
    let response = try_connect(&request_url);
    start_request(&request_url, duration, &output_dir);
}

fn init_log() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
        ]
    ).unwrap();
}

fn build_command_match() -> (String, String, String, Duration) {
    let matches = App::new("RC-Collector")
        .version("0.1.0")
        .author("Sam")
        .about("Syzkaller raw cover collector via http request.")
        .arg(Arg::with_name("ip")
            .short("i")
            .long("ip")
            .takes_value(true).required(true)
            .help("Ip address of Syzkaller http server"))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port").required(true)
            .takes_value(true)
            .help("Port number of Syzkaller http server"))
        .arg(Arg::with_name("output_dic")
            .short("o")
            .long("output_dir")
            .takes_value(true)
            .default_value(".")
            .help("Output dir,default '.'"))
        .arg(Arg::with_name("duration")
            .short("du")
            .long("duration")
            .takes_value(true)
            .default_value("3")
            .help("Raw cover collecting duration / minute"))
        .get_matches();
    (matches.value_of("ip").unwrap().to_string(),
     matches.value_of("port").unwrap().to_string(),
     matches.value_of("output_dic").unwrap().to_string(),
     Duration::from_secs(matches.value_of("duration").unwrap().parse().unwrap()))
}

fn build_url(ip: &str, port: &str) -> Url {
    let request_url = format!("http://{}:{}/{}", ip, port, RAW_COVER);
    let mut request_url = match Url::parse(&request_url) {
        Ok(req) => req,
        Err(error) => {
            error!("Error:{}", error);
            exit(1)
        }
    };
    request_url
}

fn try_connect(syzkaller_server_url: &str) -> Response {
    let mut request = request_builder(syzkaller_server_url).send();
    match request {
        Ok(resp) => {
            info!("Connection Test Passed");
            resp
        }
        Err(err) => {
            error!("Error while TEST CONNECTION:\n{}", err);
            exit(1)
        }
    }
}

fn start_request(request_url: &str, duration: Duration, output_dir: &str) {
    let (tx, rx) = mpsc::channel();
    let persistence_thread = start_persistence_task(rx);
//    let stop_sender = tx.clone();
//    ctrlc::set_handler(move || {
//        println!("Waiting for persistence task...");
//        while let Ok(_) = stop_sender.send(None) {
//            sleep(Duration::from_millis(200));
//        }
//        exit(0);
//    });
    loop {
        sleep(duration);
        match request_builder(request_url).send() {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    info!("ID:{:?} Got Data from syzkaller", std::thread::current().id());
                    match tx.send(Some(response)) {
                        Ok(_) => (),
                        Err(_) => {
                            warn!("ID:{:?} Sending data to persistence task failed,assume persistence thread closed", std::thread::current().id());
                            return;
                        }   // assume persistence thread closed
                    }
                } else {
                    tx.send(None);
                    warn!("ID:{:?} Error response code,waiting for persistence task finished", std::thread::current().id());
                    persistence_thread.join();
                    return;
                }
            }
            Err(error) => {
                eprintln!("ID:{:?} Error while sending request:\n{}", std::thread::current().id(), error);
                tx.send(None);
                persistence_thread.join();
                return;
            }
        }
    }
}

fn start_persistence_task(rx: Receiver<Option<Response>>) -> JoinHandle<()> {
    spawn(move || {
        info!("ID:{:?} Persistence Task start", std::thread::current().id());
        let mut last_time = Instant::now();
        loop {
            match rx.recv() {
                Ok(msg) =>
                    match msg {
                        Some(response) => {
                            info!("ID:{:?} Got data,{}", std::thread::current().id(), response.status());
                        }
                        None => {
                            info!("ID:{:?} Persistence task finished.Data saved ok", std::thread::current().id());
                            return;
                        }
                    }
                Err(_) => {
                    error!("ID:{:?} Error while getting msg from channel", std::thread::current().id());
                    exit(1);
                }
            }
        }
    })
}

#[inline]
fn request_builder(request_url: &str) -> RequestBuilder {
    Client::builder()
        .timeout(Duration::from_secs(3))
        .build().unwrap()
        .get(request_url)
}