use std::{fmt, fs};
use std::fmt::Display;
use std::io::Error;
use std::net::{IpAddr, Ipv4Addr};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread::{JoinHandle, sleep, spawn};
use std::time::{Duration, Instant};

use chrono::format::Pad;
use clap::{App, Arg, ArgMatches};
use log::{error, info, LevelFilter};
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use simplelog::{CombinedLogger, Config, TermLogger};
use url::{OpaqueOrigin, Url};
use url::ParseError;

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
    OutputDirError(Error),
}

#[derive(Debug)]
pub struct ConfigError {
    error_type: ErrorType
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ErrorType::*;
        match self.error_type {
            UrlParseError(ref reason) => write!(f, "UrlParseError:{}", reason),
            DurationParseError(ref reason) => write!(f, "DurationParseError:{}", reason),
            OutputDirError(ref reason) => write!(f, "OutputDirError:{}", reason),
        }
    }
}

impl ConfigError {
    pub fn new(error_type: ErrorType) -> Self {
        ConfigError { error_type }
    }
}

impl AppConfig {
    pub fn new(url: &str, duration: &str, output_dir: &str) -> Result<AppConfig, ConfigError> {
        let request_url = match Url::parse(url) {
            Ok(mut u) => u,
            Err(reason) => return Err(ConfigError::new(ErrorType::UrlParseError(reason)))
        };
        let duration = match duration.parse::<u64>() {
            Ok(duration_u64) => Duration::from_secs(duration_u64),
            Err(error) => {
                return Err(ConfigError::new(ErrorType::DurationParseError(error)));
            }
        };
        let output_dir = PathBuf::from(output_dir);
        if !output_dir.exists() {
            info!("output dir{:?} not exists,creating...", output_dir.as_os_str());
            match fs::create_dir(&output_dir) {
                Ok(_) => (),
                Err(error) => {
                    return Err(ConfigError::new(ErrorType::OutputDirError(error)));
                }
            }
        }
        AppConfig::test_connection(request_url.as_str());
        Ok(AppConfig {
            request_url,
            output_dir,
            duration,
        })
    }

    fn test_connection(request_url: &str) {
        let response = Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap()
            .get(request_url).send();
        match response {
            Ok(_) => info!("Connection Test Passed"),
            Err(error) => {
                error!("Connection Test Failed:/n{}", error);
                exit(exitcode::DATAERR);
            }
        }
    }
}