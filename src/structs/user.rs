#[derive(Debug)]
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

pub fn build_user(email: String, username: String) -> User {
    // Field init shorthand because the email and username parameters have the same name as struct fields
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
