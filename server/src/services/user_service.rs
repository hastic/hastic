use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use std::iter::repeat_with;

pub type AccessToken = String;

const TOKEN_LENGTH: usize = 20;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub struct UserService {
    tokens: HashSet<AccessToken>,
}

impl UserService {
    pub fn new() -> UserService {
        UserService {
            tokens: HashSet::new(),
        }
    }
    pub fn login(&mut self, user: &User) -> Option<AccessToken> {
        if user.username == "admin" && user.password == "admin" {
            let token: AccessToken = repeat_with(fastrand::alphanumeric)
                .take(TOKEN_LENGTH)
                .collect();
            self.tokens.insert(token.to_owned());
            return Some(token);
        }
        return None;
    }
    pub fn check_token(&self, username: &String, token: &AccessToken) -> bool {
        return self.tokens.contains(token);
    }
}
