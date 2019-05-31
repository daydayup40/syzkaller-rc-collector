use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::Local;
use reqwest::{Client, Response};
use select::document::Document;
use select::predicate::{Name, Predicate};
use url::Url;

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

pub fn request_and_parse(request_url: &Url, time_out: Duration) -> Result<String, RequestorParseError> {
    let mut response = Client::builder()
        .timeout(time_out)
        .build().expect("unexpected exception happened, contact to sam")
        .get(request_url.as_str())
        .send()?;
    parse(&mut response).map_err(|err| err.into())
}

fn parse(response: &mut Response) -> Result<String, std::io::Error> {
//    let document = Document::from_read(response)?;
//    let node = document.find(Name("table")
//        .descendant(Name("p")))
//        .take(1).next().unwrap().text();
//    Some(node);
//    unimplemented!()
    Ok(response.text().unwrap())
}

#[inline]
#[allow(dead_code)]
fn get_lines_number(lines: &str) -> usize {
    lines.lines().count()
}

pub fn persist_on_time(data: &str, path: &Path) -> Result<PathBuf, std::io::Error> {
    let now = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    if data.is_empty() {
        eprintln!("[{}]:Warning writing empty data to path:{:?}", now, path);
    }
    let file_path = path.join(Path::new(&now));
    let f = File::create(file_path.as_path())?;
    BufWriter::new(f)
        .write_all(data.as_bytes())
        .map(|_| file_path)
}

#[cfg(test)]
mod test {
    use std::env::temp_dir;
    use std::path::PathBuf;

    use super::{get_lines_number, persist_on_time};

    #[test]
    fn test_get_line_length() {
        let empty_data = "";
        let one = "one line";
        let two_lines = "one\ntwo";
        let three_lines = "one\ntwo\ntts";
        assert_eq!(0, get_lines_number(empty_data));
        assert_eq!(1, get_lines_number(one));
        assert_eq!(2, get_lines_number(two_lines));
        assert_eq!(3, get_lines_number(three_lines));
    }

    #[test]
    fn test_persist_on_time() {
        use std::fs::metadata;
        let empty_data = "";
        let wrong_path = PathBuf::from("///////");
        let pwd = PathBuf::from(".");
        let data = "some data";
        let temp_dir = temp_dir();
        assert!(persist_on_time(empty_data, &wrong_path).is_err());
        let save_file_length = persist_on_time(data, &temp_dir)
            .map(|path| metadata(path).expect("persist_on_time return path that should be already created but can not be open").len())
            .expect("file creation failed");
        assert_eq!(save_file_length, data.len() as u64);
        let save_file_length = persist_on_time(data, &pwd)
            .map(|path| metadata(path).expect("persist_on_time return path that should be already created but can not be open").len())
            .expect("file creation failed");
        assert_eq!(save_file_length, data.len() as u64);
    }
}