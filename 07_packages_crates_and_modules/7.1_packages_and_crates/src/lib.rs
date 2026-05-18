pub mod auth_utils;

mod database;

pub fn authenticate(cred: &auth_utils::models::Credentials) {
    use auth_utils::login;
    use database::{Status::Connected, connect_db};

    if matches!(connect_db(), Connected) {
        login(cred);
    }
}
