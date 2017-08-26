use handlers::prelude::*;

pub fn handle_static(root: &'static str, toplevel: TopLevel, sublevel: SubLevel)
        -> impl Fn(&mut Request) -> IronResult<Response> {
    move |req| {
        let contents = slurp_bytes(format!("static/{}{}", root, req.url.path().join("/")))?;
        resp(contents, toplevel.clone(), sublevel.clone())
    }
}
