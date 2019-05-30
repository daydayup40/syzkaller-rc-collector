use chrono::Local;
use reqwest::{Client, Response};
use select::document::Document;
use select::predicate::{Name, Predicate};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Duration;

pub enum RequestorParseError {
    RequestError(reqwest::Error),
    DataParseError(std::io::Error),
}

impl Display for RequestorParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            RequestorParseError::RequestError(ref reqwest_eeor) =>
                write!(f, "RequestError:{}", reqwest_eeor),
            RequestorParseError::DataParseError(ref io_error) =>
                write!(f, "DataParseError:{}", io_error),
        }
    }
}

impl From<reqwest::Error> for RequestorParseError {
    fn from(reqwest_error: reqwest::Error) -> Self {
        RequestorParseError::RequestError(reqwest_error)
    }
}

impl From<std::io::Error> for RequestorParseError {
    fn from(io_error: std::io::Error) -> Self {
        RequestorParseError::DataParseError(io_error)
    }
}

pub fn request_and_parse(request_url: &str, time_out: Duration) -> Result<String, RequestorParseError> {
    let response = Client::builder()
        .timeout(time_out)
        .build().unwrap()
        .get(request_url)
        .send()?;
    parse(response).map_err(|err| err.into())
}

fn parse(response: Response) -> Result<String, std::io::Error> {
    let document = Document::from_read(response)?;
    let node = document.find(Name("table")
        .descendant(Name("p")))
        .take(1).next().unwrap().text();
    Some(node);
    unimplemented!()
}


pub fn persistence_on_time(data: &str, path: &Path) -> Result<(), std::io::Error> {
    let now = Local::now().format("%Y-%m-%d][%H:%M:%S").to_string();
    let file_path = path.join(Path::new(&now));
    let f = File::create(file_path.as_path())?;
    BufWriter::new(f).write_all(data.as_bytes())
}

pub struct Collector {}

impl Collector {
    pub fn run(&self) {
        unimplemented!()
    }
}