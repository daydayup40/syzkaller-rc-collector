use std::time::{Duration, Instant};

use log::{error, info, warn};
use reqwest::{Client, Response};
use select::document::Document;
use select::predicate::{Attr, Class, Element, Name, Predicate};

pub struct Collector {}

pub struct Requestor {}

impl Requestor {
    pub fn request(&self, request_url: &str) -> Option<String> {
        let mut request_result = Client::builder()
            .timeout(Duration::from_secs(3))
            .build().unwrap()
            .get(request_url).send();
        match request_result {
            Ok(mut response) => {
                self.parse(response)
            }
            Err(error) => {
                error!("{}", error);
                None
            }
        }
    }
    fn parse(&self, response: Response) -> Option<String> {
        let document = Document::from_read(response).unwrap();
        let node = document.find(Name("table")
            .descendant(Name("p")))
            .take(1).next().unwrap().text();
        Some(node);
        unimplemented!()
    }
}
//
//impl Collector {
//    pub fn collect(request_url:&str,duration :Duration) {
//        loop {
//            sleep(duration);
//            match request_builder(request_url).send() {
//                Ok(response) => {
//                    if response.status() == StatusCode::OK {
//                        info!("ID:{:?} Got Data from syzkaller", std::thread::current().id());
//                        match tx.send(Some(response)) {
//                            Ok(_) => (),
//                            Err(_) => {
//                                warn!("ID:{:?} Sending data to persistence task failed,assume persistence thread closed", std::thread::current().id());
//                                return;
//                            }   // assume persistence thread closed
//                        }
//                    } else {
//                        tx.send(None);
//                        warn!("ID:{:?} Error response code,waiting for persistence task finished", std::thread::current().id());
//                        persistence_thread.join();
//                        return;
//                    }
//                }
//                Err(error) => {
//                    eprintln!("ID:{:?} Error while sending request:\n{}", std::thread::current().id(), error);
//                    tx.send(None);
//                    persistence_thread.join();
//                    return;
//                }
//            }
//        }
//    }
//
//}
//
//struct Persistence {
//    thread_handler: JoinHandle<()>
//}
//
//impl Persistence {
//    pub fn run(&mut self) {
//        self.thread_handler = spawn(move || {
//            info!("ID:{:?} Persistence Task start", std::thread::current().id());
//            let mut last_time = Instant::now();
//            loop {
//                match rx.recv() {
//                    Ok(msg) =>
//                        match msg {
//                            Some(response) => {
//                                info!("ID:{:?} Got data,{}", std::thread::current().id(), response.status());
//                            }
//                            None => {
//                                info!("ID:{:?} Persistence task finished.Data saved ok", std::thread::current().id());
//                                return;
//                            }
//                        }
//                    Err(_) => {
//                        error!("ID:{:?} Error while getting msg from channel", std::thread::current().id());
//                        exit(exitcode::SOFTWARE);
//                    }
//                }
//            }
//        });
//    }
//    pub fn wait(&self){
//        self.thread_handler.join();
//    }
//}

//
//
//fn start_request(request_url: &str, duration: Duration, output_dir: &str) {
//    let (tx, rx) = mpsc::channel();
//    let persistence_thread = start_persistence_task(rx);
//    ctrlc::set_handler(move || {
//        unimplemented!()
//    });
//    loop {
//        sleep(duration);
//        match request_builder(request_url).send() {
//            Ok(response) => {
//                if response.status() == StatusCode::OK {
//                    info!("ID:{:?} Got Data from syzkaller", std::thread::current().id());
//                    match tx.send(Some(response)) {
//                        Ok(_) => (),
//                        Err(_) => {
//                            warn!("ID:{:?} Sending data to persistence task failed,assume persistence thread closed", std::thread::current().id());
//                            return;
//                        }   // assume persistence thread closed
//                    }
//                } else {
//                    tx.send(None);
//                    warn!("ID:{:?} Error response code,waiting for persistence task finished", std::thread::current().id());
//                    persistence_thread.join();
//                    return;
//                }
//            }
//            Err(error) => {
//                eprintln!("ID:{:?} Error while sending request:\n{}", std::thread::current().id(), error);
//                tx.send(None);
//                persistence_thread.join();
//                return;
//            }
//        }
//    }
//}
//
//fn start_persistence_task(rx: Receiver<Option<Response>>) -> JoinHandle<()> {
//    spawn(move || {
//        info!("ID:{:?} Persistence Task start", std::thread::current().id());
//        let mut last_time = Instant::now();
//        loop {
//            match rx.recv() {
//                Ok(msg) =>
//                    match msg {
//                        Some(response) => {
//                            info!("ID:{:?} Got data,{}", std::thread::current().id(), response.status());
//                        }
//                        None => {
//                            info!("ID:{:?} Persistence task finished.Data saved ok", std::thread::current().id());
//                            return;
//                        }
//                    }
//                Err(_) => {
//                    error!("ID:{:?} Error while getting msg from channel", std::thread::current().id());
//                    exit(1);
//                }
//            }
//        }
//    })
//}
//
//#[inline]
//fn request_builder(request_url: &str) -> RequestBuilder {
//    Client::builder()
//        .timeout(Duration::from_secs(3))
//        .build().unwrap()
//        .get(request_url)
//}