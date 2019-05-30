use std::{fmt, fs};
use std::io;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::time::Duration;

use log::info;
use reqwest::{Client, Response};
use url::ParseError;
use url::Url;

pub mod collector;

#[derive(Debug)]
pub struct AppConfig {
    pub request_url: Url,
    pub output_dir: PathBuf,
    pub duration: Duration,
}

#[derive(Debug)]
pub enum ErrorType {
    UrlParseError(ParseError),
    DurationParseError(ParseIntError),
    OutputDirError(io::Error),
    ConnectionError(reqwest::Error),
}


impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ErrorType::*;
        match self {
            UrlParseError(ref reason) => write!(f, "UrlParseError:{}", reason),
            DurationParseError(ref reason) => write!(f, "DurationParseError:{}", reason),
            OutputDirError(ref reason) => write!(f, "OutputDirError:{}", reason),
            ConnectionError(ref reason) => write!(f, "ConnectionError:{}", reason)
        }
    }
}


impl AppConfig {
    pub fn new(url: &str, duration: &str, output_dir: &str) -> Result<AppConfig, ErrorType> {
        let request_url = match Url::parse(url) {
            Ok(u) => u,
            Err(reason) => return Err(ErrorType::UrlParseError(reason))
        };
        let duration = match duration.parse::<u64>() {
            Ok(duration_u64) => Duration::from_secs(duration_u64),
            Err(error) => {
                return Err(ErrorType::DurationParseError(error));
            }
        };
        let output_dir = PathBuf::from(output_dir);
        if !output_dir.exists() {
            info!("output dir{:?} not exists,creating...", output_dir.as_os_str());
            match fs::create_dir(&output_dir) {
                Ok(_) => (),
                Err(error) => {
                    return Err(ErrorType::OutputDirError(error));
                }
            }
        }
        match AppConfig::test_connection(request_url.as_str()) {
            Ok(_) => Ok(AppConfig {
                request_url,
                output_dir,
                duration,
            }),
            Err(error) => Err(ErrorType::ConnectionError(error))
        }
    }

    fn test_connection(request_url: &str) -> Result<Response, reqwest::Error> {
        Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap()
            .get(request_url).send()
    }
}