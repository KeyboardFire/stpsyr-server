use handlers::prelude::*;

pub fn handle_css(req: &mut Request) -> IronResult<Response> {
    let contents = slurp(format!("static/css/{}{}", "dark-", req.url.path().join("/")))?;
    resp(contents, TopLevel::Text, SubLevel::Css)
}
