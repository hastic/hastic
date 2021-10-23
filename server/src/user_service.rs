use serde::{ Deserialize, Serialize };


pub type AccessToken = String;

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
    pub fn login(user: &User) -> Option<AccessToken> {
        if user.username == "admin" && user.password == "admin" {
            return Some("asdsadsad".to_string());
        }
        return None;
    }
}