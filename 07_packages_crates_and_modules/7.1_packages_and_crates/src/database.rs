use crate::auth_utils;

pub enum Status {
    Connected,
    Interrupted,
}

pub fn connect_db() -> Status {
    // TODO: connect to actual database
    Status::Connected
}

pub fn get_usr(cred: &auth_utils::models::Credentials) {
    // TODO: fetch the user from database
}
