use handlers::prelude::*;

use iron_sessionstorage;
use iron_sessionstorage::traits::*;
use persistent;
use Db;

struct SessionToken(String);
impl iron_sessionstorage::Value for SessionToken {
    fn get_key() -> &'static str { "sessiontoken" }
    fn into_raw(self) -> String { self.0 }
    fn from_raw(val: String) -> Option<Self> { Some(SessionToken(val)) }
}

pub fn handle_html(name: &'static str)
        -> impl Fn(&mut Request) -> IronResult<Response> {
    move |req| {
        let ext = if match name {
            "users" | "games" => !req.url.path()[0].is_empty(),
            _ => false
        } { "-ext" } else { "" };

        let mut contents = format!("{}{}{}",
            slurp("static/html/header.html")?,
            slurp(format!("static/html/{}{}.html", name, ext))?,
            slurp("static/html/footer.html")?);

        let conn = req.get::<persistent::Write<Db>>().unwrap();
        let conn = conn.lock().unwrap();

        let (userid, username) = match req.session().get::<SessionToken>()? {
            Some(token) => {
                let rows = conn.query("
                    SELECT u.id, u.username
                    FROM users u
                    WHERE t.token = $1
                    INNER JOIN tokens t
                    ON u.id = t.userid", &[&token.0]).unwrap();
                if rows.is_empty() {
                    req.session().clear()?;
                    (None, None)
                } else {
                    let row = rows.get(0);
                    (Some(row.get::<usize, i32>(0)),
                     Some(row.get::<usize, String>(1)))
                }
            },
            None => (None, None)
        };

        let mut start = 0;
        while let Some(pos) = (&contents[start..]).find('~') {
            start += pos;
            let repl = match &contents[start+1..start+4] {
                "acc" => if let Some(ref username) = username {
                    format!("logged in as <a href='/users/{}'>{}</a>",
                            userid.unwrap(), username)
                } else {
                    "<a href='/login'>login</a>".to_string()
                },
                _ => panic!("weird thing")
            };
            contents.splice(start..start+5, &repl);
            start += repl.len();
        }

        resp(contents, TopLevel::Text, SubLevel::Html)
    }
}
