#![feature(conservative_impl_trait)]

extern crate iron;
use iron::prelude::*;
use iron::mime::{TopLevel, SubLevel};

extern crate router;
use router::Router;

extern crate mount;
use mount::Mount;

mod handlers;

extern crate simplelog;
use simplelog::{CombinedLogger, TermLogger, WriteLogger, LogLevelFilter, Config};

extern crate logger;
use logger::Logger;

use std::fs::OpenOptions;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Info, Config::default(),
                OpenOptions::new().append(true)
                                  .create(true)
                                  .open("log.txt")
                                  .unwrap())
        ]
    ).unwrap();

    let mut router = Router::new();
    router.get("/", handlers::handle_html("home"), "home");

    let mut mount = Mount::new();
    mount.mount("/css", handlers::handle_css);
    mount.mount("/js", handlers::handle_static("js/", TopLevel::Text, SubLevel::Javascript));
    mount.mount("/png", handlers::handle_static("png/", TopLevel::Image, SubLevel::Png));
    mount.mount("/svg", handlers::handle_static("svg/", TopLevel::Image, SubLevel::Ext("svg+xml".to_string())));
    mount.mount("/", router);

    let mut chain = Chain::new(mount);
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}
