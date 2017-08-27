use handlers::prelude::*;

use params::{Params, Value};
use iron_sessionstorage::traits::*;
use handlers::html::SessionToken;
use persistent;
use Db;

extern crate crypto;
use self::crypto::bcrypt;

extern crate rand;
use self::rand::{Rng, OsRng};

pub fn handle_login(req: &mut Request) -> IronResult<Response> {
    let (username, password) = {
        let map = req.get_ref::<Params>().unwrap();
        (match map.find(&["username"]) {
            Some(&Value::String(ref username)) => username.clone(),
            _ => String::new()
         }, match map.find(&["password"]) {
            Some(&Value::String(ref password)) => password.clone(),
            _ => String::new()
         })
    };

    let conn = req.get::<persistent::Write<Db>>().unwrap();
    let conn = conn.lock().unwrap();

    let rows = conn.query("
        SELECT u.salt, u.hash, u.id, t.token
        FROM users u
        LEFT JOIN tokens t
        ON u.id = t.userid
        WHERE username = $1", &[&username]).unwrap();
    if rows.is_empty() {
        return redir("/login");
    }

    let row = rows.get(0);
    let salt: Vec<u8> = row.get(0);
    let salt = salt.as_slice();
    // I can't believe I actually have to do this
    let salt = [salt[0],  salt[1],  salt[2],  salt[3],
                salt[4],  salt[5],  salt[6],  salt[7],
                salt[8],  salt[9],  salt[10], salt[11],
                salt[12], salt[13], salt[14], salt[15]];
    let hash: Vec<u8> = row.get(1);
    if hash != hash_pwd(salt, &password) {
        return redir("/login");
    }

    // login successful
    let token = if let Some(token) = row.get(3) {
        token
    } else {
        let mut rng = OsRng::new().unwrap();
        let token: String = rng.gen_ascii_chars().take(64).collect();
        conn.execute("
            INSERT INTO tokens (userid, token)
            VALUES ($1, $2)", &[&row.get::<usize, i32>(2), &token]).unwrap();
        token
    };
    req.session().set::<SessionToken>(SessionToken(token))?;

    redir("/")
}

pub fn handle_register(req: &mut Request) -> IronResult<Response> {
    let (username, password) = {
        let map = req.get_ref::<Params>().unwrap();
        (match map.find(&["username"]) {
            Some(&Value::String(ref username)) => username.clone(),
            _ => String::new()
         }, match map.find(&["password"]) {
            Some(&Value::String(ref password)) => password.clone(),
            _ => String::new()
         })
    };

    let conn = req.get::<persistent::Write<Db>>().unwrap();
    let conn = conn.lock().unwrap();

    let mut salt = [0u8; 16];
    let mut rng = OsRng::new().unwrap();
    rng.fill_bytes(&mut salt);
    let mut salt_vec = Vec::with_capacity(16);
    salt_vec.write(&salt).unwrap();
    let hash = hash_pwd(salt, &password);

    let register_query = conn.query("
        INSERT INTO users (username, salt, hash)
        VALUES ($1, $2, $3)
        RETURNING id",
        &[&username, &salt_vec, &hash]);
    let success = register_query.is_ok();
    let userid: i32 = if success {
        register_query.unwrap().get(0).get(0)
    } else {
        return redir("/login");
    };

    let token: String = rng.gen_ascii_chars().take(64).collect();
    conn.execute("
        INSERT INTO tokens (userid, token)
        VALUES ($1, $2)", &[&userid, &token]).unwrap();
    req.session().set::<SessionToken>(SessionToken(token))?;

    redir("/")
}

fn hash_pwd(salt: [u8; 16], password: &String) -> Vec<u8> {
    let mut result = [0u8; 24];
    let password: &[u8] = &password.as_bytes().into_iter().cloned().take(72)
        .collect::<Vec<u8>>()[..];
    let empty_password = &[0];
    bcrypt::bcrypt(10, &salt, if password.is_empty() { empty_password }
                   else { password }, &mut result);
    let mut v = Vec::with_capacity(24);
    v.write(&result).unwrap();
    v
}
