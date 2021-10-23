use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String
}

pub struct UserService {

}

impl UserService {
    pub fn new() -> UserService {
        UserService{}
    }

}