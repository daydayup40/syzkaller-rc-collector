use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

use log::info;
use url::ParseError;
use url::Url;

use self::util::{persist_on_time, request_and_parse, RequestorParseError};

mod util;

#[derive(Debug)]
pub struct Collector {
    pub request_url: Url,
    pub output_dir: PathBuf,
    pub duration: Duration,
}


impl Collector {
    pub fn new(url: &str, duration: &str, output_dir: &str) -> Result<Collector, ArgumentsParseError> {
        let request_url = Url::parse(url)?;
        let duration = Duration::from_secs(duration.parse::<u64>()?);
        let output_dir = PathBuf::from(output_dir);
        Collector::dir_exist_or_create(output_dir.as_path())?;
        Ok(Collector {
            request_url,
            duration,
            output_dir,
        })
    }
    pub fn run(&self) -> Result<(), Error> {
        info!("Start Collecting");
        loop {
            sleep(self.duration);
            let data = request_and_parse(&self.request_url, Duration::from_secs(3))?;
            let saved_file = persist_on_time(&data, &self.output_dir)?;
            info!("Get file,{:?}", &saved_file)
        }
    }

    fn dir_exist_or_create(output_dir: &Path) -> Result<(), std::io::Error> {
        if !output_dir.exists() {
            std::fs::create_dir(output_dir)?
        };
        Ok(())
    }
}


pub enum Error {
    RequestOrParseError(RequestorParseError),
    IOError(std::io::Error),
}

impl From<RequestorParseError> for Error {
    fn from(error: RequestorParseError) -> Self {
        Error::RequestOrParseError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::RequestOrParseError(ref error) => write!(f, "RequestOrParseError:{}", error),
            Error::IOError(ref error) => write!(f, "IOError:{}", error)
        }
    }
}


#[derive(Debug)]
pub enum ArgumentsParseError {
    UrlParseError(ParseError),
    DurationParseError(ParseIntError),
    OutputDirError(std::io::Error),
    ConnectionError(reqwest::Error),
}

impl From<ParseError> for ArgumentsParseError {
    fn from(error: ParseError) -> Self {
        ArgumentsParseError::UrlParseError(error)
    }
}

impl From<ParseIntError> for ArgumentsParseError {
    fn from(error: ParseIntError) -> Self {
        ArgumentsParseError::DurationParseError(error)
    }
}

impl From<std::io::Error> for ArgumentsParseError {
    fn from(error: std::io::Error) -> Self {
        ArgumentsParseError::OutputDirError(error)
    }
}

impl From<reqwest::Error> for ArgumentsParseError {
    fn from(error: reqwest::Error) -> Self {
        ArgumentsParseError::ConnectionError(error)
    }
}

impl fmt::Display for ArgumentsParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ArgumentsParseError::*;
        match self {
            UrlParseError(ref reason) => write!(f, "UrlParseError:{}", reason),
            DurationParseError(ref reason) => write!(f, "DurationParseError:{}", reason),
            OutputDirError(ref reason) => write!(f, "OutputDirError:{}", reason),
            ConnectionError(ref reason) => write!(f, "ConnectionError:{}", reason)
        }
    }
}
