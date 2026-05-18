use auth_service::{auth_utils::models::Credentials, authenticate};

fn main() {
    let user1 = Credentials {
        username: "JohnDoe".to_string(),
        password: "12345678".to_string(),
    };

    authenticate(&user1);
}
