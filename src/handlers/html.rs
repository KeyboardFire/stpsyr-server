use handlers::prelude::*;

pub fn handle_html(name: &'static str)
        -> impl Fn(&mut Request) -> IronResult<Response> {
    move |_| {
        let contents = slurp(format!("static/html/{}.html", name))?;
        resp(contents, TopLevel::Text, SubLevel::Html)
    }
}
