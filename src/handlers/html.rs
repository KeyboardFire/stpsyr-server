use handlers::prelude::*;

pub fn handle_html(name: &'static str)
        -> impl Fn(&mut Request) -> IronResult<Response> {
    move |_| {
        let contents = format!("{}{}{}",
            slurp("static/html/header.html")?,
            slurp(format!("static/html/{}.html", name))?,
            slurp("static/html/footer.html")?);
        resp(contents, TopLevel::Text, SubLevel::Html)
    }
}
