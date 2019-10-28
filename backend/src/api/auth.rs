use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use super::models::User;

pub extern crate jwt;

use crypto::sha2::Sha256;
use self::jwt::{Header, Registered, Token};

pub struct AuthOption {
    pub claims: Registered,
    pub user: Option<User>,
}

fn read_token(key: &str) -> Result<Registered, String> {
    let token = Token::<Header, Registered>::parse(key)
        .map_err(|_| "Unable to parse key".to_string())?;

    if token.verify(b"secret_key", Sha256::new()) {
        Ok(token.claims)
    } else {
        Err("Token not valid".to_string())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthOption {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthOption, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        
        match read_token(keys[0]) {
            Ok(claims) => Outcome::Success(AuthOption { claims, user: None }),
            Err(_) => Outcome::Forward(())
        }
    }
}