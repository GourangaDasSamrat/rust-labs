use crate::database;

pub mod models;

pub fn login(cred: &models::Credentials) {
    // TODO: fetch the user
    database::get_usr(cred);
}
