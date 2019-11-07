use crate::api::models::User;
use chrono::{prelude::*, Duration};
use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};
use rocket::{
    request::{self, FromRequest, Request},
    Outcome,
};
use std::convert::TryInto;
use std::env::{self, VarError};

pub struct AuthOption {
    pub claims: Registered,
    pub user: Option<User>,
}

const DEFAULT_LIFETIME: i64 = 60 * 60 * 24 * 7;

pub fn create_token(user: &User) -> Token<Header, Registered> {
    let lifetime: i64 = env::var("JWT_LIFETIME")
        .and_then(|v| v.parse().map_err(|_| VarError::NotPresent))
        .unwrap_or(DEFAULT_LIFETIME);
    let expiration = (Utc::now() + Duration::seconds(lifetime)).timestamp();

    let claims = Registered {
        iss: Some("Demeter".to_string()),
        sub: Some(user.id.to_string()),
        aud: Some("Demeter Panel".to_string()),
        exp: Some(expiration.try_into().unwrap()),
        iat: Some(Utc::now().timestamp().try_into().unwrap()),
        // TODO: Determine if we want to implement the jti claim
        ..Default::default()
    };

    Token::<Header, Registered>::new(Default::default(), claims)
}

fn read_token(key: &str) -> Result<Registered, String> {
    let token =
        Token::<Header, Registered>::parse(key).map_err(|_| "Unable to parse key".to_string())?;

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
            Err(_) => Outcome::Forward(()),
        }
    }
}
