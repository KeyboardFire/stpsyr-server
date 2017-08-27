#![feature(conservative_impl_trait)]
#![feature(splice)]

extern crate iron;
use iron::prelude::*;
use iron::mime::{TopLevel, SubLevel};
use iron::typemap::Key;

extern crate router;
use router::Router;

extern crate mount;
use mount::Mount;

mod handlers;

extern crate simplelog;
use simplelog::{CombinedLogger, TermLogger, WriteLogger, LogLevelFilter, Config};

extern crate logger;
use logger::Logger;

extern crate iron_sessionstorage;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

extern crate persistent;

extern crate postgres;
use postgres::{Connection, TlsMode};

use std::fs::{File, OpenOptions};
use std::io::Read;
use std::env;

pub struct Db;
impl Key for Db { type Value = Connection; }

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
    router.get("/login", handlers::handle_html("login"), "login");

    let mut mount = Mount::new();
    mount.mount("/users", handlers::handle_html("users"));
    mount.mount("/games", handlers::handle_html("games"));
    mount.mount("/css", handlers::handle_css);
    mount.mount("/js", handlers::handle_static("js/", TopLevel::Text, SubLevel::Javascript));
    mount.mount("/png", handlers::handle_static("png/", TopLevel::Image, SubLevel::Png));
    mount.mount("/svg", handlers::handle_static("svg/", TopLevel::Image, SubLevel::Ext("svg+xml".to_string())));
    mount.mount("/", router);

    let mut chain = Chain::new(mount);

    let (logger_before, logger_after) = Logger::new(None);

    let mut secret_file = File::open("secret")
        .expect("please provide a secret in file 'secret' for signing cookies");
    let mut secret = Vec::new();
    secret_file.read_to_end(&mut secret).unwrap();

    let conn = Connection::connect("postgres://postgres@localhost",
                                   TlsMode::None).unwrap();
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("-r")) {
        conn.batch_execute("
        DROP TABLE IF EXISTS users CASCADE;
        DROP TABLE IF EXISTS tokens CASCADE;

        CREATE TABLE users (
        id          SERIAL PRIMARY KEY,
        username    TEXT NOT NULL UNIQUE,
        salt        BYTEA NOT NULL,
        hash        BYTEA NOT NULL
        );

        CREATE TABLE tokens (
        userid      INT NOT NULL,
        token       TEXT NOT NULL
        );
        ").unwrap();
    }

    chain.link_before(logger_before);
    chain.link_around(SessionStorage::new(SignedCookieBackend::new(secret)));
    chain.link(persistent::Write::<Db>::both(conn));
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();
}
