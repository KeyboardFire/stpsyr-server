use handlers::prelude::*;

use iron_sessionstorage;

struct SessionToken(String);
impl iron_sessionstorage::Value for SessionToken {
    fn get_key() -> &'static str { "sessiontoken" }
    fn into_raw(self) -> String { self.0 }
    fn from_raw(val: String) -> Option<Self> { Some(SessionToken(val)) }
}

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
