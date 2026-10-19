use actix_web::{dev::Payload, error::ErrorUnauthorized, web, Error, FromRequest, HttpRequest};
use futures_util::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    server::{state::AppState, utils::read_whitelist},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct FirebaseUser {
    pub uid: String,
}

fn get_token_from_header(header: &str) -> Option<String> {
    let prefix_len = "Bearer ".len();

    match header.len() {
        l if l < prefix_len => None,
        _ => Some(header[prefix_len..].to_string()),
    }
}

fn validate_user(uid: &String, config: Config) -> bool {
    match config.whitelist {
        Some(whitelist) => match read_whitelist(whitelist) {
            Ok(users) => users.contains(&uid),
            Err(_) => false,
        },
        None => true, // No whitelist specified, allow all users
    }
}

impl FromRequest for FirebaseUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap().clone();

        let data = match data.lock() {
            Ok(guard) => guard,
            Err(e) => return err(actix_web::error::ErrorInternalServerError(e.to_string())),
        };

        let config = data.config.clone();
        let jwk_auth = &data.auth;

        if let Some(jwk_auth) = jwk_auth {
            let token = match req.headers().get("Authorization") {
                Some(auth_header) => match auth_header.to_str() {
                    Ok(v) => get_token_from_header(v),
                    _ => return err(ErrorUnauthorized("Could not parse auth header")),
                },
                _ => return err(ErrorUnauthorized("Could not parse auth header")),
            };

            if token.is_none() {
                return err(ErrorUnauthorized("Could not parse auth header"));
            }

            let _token = token.unwrap();

            let token_data = jwk_auth.verify(&_token);

            match token_data {
                Some(data) => match validate_user(&data.claims.sub, config) {
                    true => ok(FirebaseUser {
                        uid: data.claims.sub,
                    }),
                    false => err(ErrorUnauthorized("user not whitelisted")),
                },
                _ => err(ErrorUnauthorized("verification failed")),
            }
        } else {
            ok(FirebaseUser { uid: String::new() })
        }
    }
}
