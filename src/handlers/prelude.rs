extern crate iron;
pub use iron::prelude::*;
pub use iron::status;
pub use iron::headers::ContentType;
pub use iron::mime::{Mime, TopLevel, SubLevel};

pub use std::fs::File;
pub use std::io::prelude::*;
use std::path::Path;

pub fn to500(e: ::std::io::Error) -> IronError {
    IronError::new(e, status::InternalServerError)
}

pub fn slurp<P: AsRef<Path>>(path: P) -> IronResult<String> {
    let mut file = File::open(path).map_err(to500)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(to500)?;
    Ok(contents)
}

pub fn resp(contents: String, toplevel: TopLevel, sublevel: SubLevel)
        -> IronResult<Response> {
    let mut resp = Response::with((status::Ok, contents));
    resp.headers.set(ContentType(Mime(toplevel, sublevel, vec![])));
    Ok(resp)
}
