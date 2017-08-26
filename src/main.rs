#![feature(conservative_impl_trait)]

extern crate iron;
use iron::prelude::*;
use iron::mime::{TopLevel, SubLevel};

extern crate router;
use router::Router;

extern crate mount;
use mount::Mount;

mod handlers;

fn main() {
    let mut router = Router::new();
    router.get("/", handlers::handle_html("home"), "home");

    let mut mount = Mount::new();
    mount.mount("/css", handlers::handle_css);
    mount.mount("/js", handlers::handle_static("js/", TopLevel::Text, SubLevel::Javascript));
    mount.mount("/png", handlers::handle_static("png/", TopLevel::Image, SubLevel::Png));
    mount.mount("/svg", handlers::handle_static("svg/", TopLevel::Image, SubLevel::Ext("svg+xml".to_string())));
    mount.mount("/", router);

    let chain = Chain::new(mount);

    Iron::new(chain).http("localhost:3000").unwrap();
}
