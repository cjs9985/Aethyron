use bcrypt::{hash, verify};
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::status;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

struct User {
    username: String,
    hashed_password: String,
}

impl User {
    fn new(username: &str, password: &str) -> Self {
        let hashed_password = hash(password, bcrypt::DEFAULT_COST).unwrap();
        User {
            username: username.to_string(),
            hashed_password,
        }
    }

    fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.hashed_password).unwrap()
    }
}

#[post("/login", format = "json", data = "<form>")]
pub async fn login(form: Form<LoginRequest>) -> Result<(), status::Custom<String>> {
    let user = find_user(&form.username).await?;
    if user.verify_password(&form.password) {
        Ok(())
    } else {
        Err(status::Custom(
            Status::Unauthorized,
            "Invalid username or password".to_string(),
        ))
    }
}

#[post("/register", format = "json", data = "<form>")]
pub async fn register(form: Form<RegisterRequest>) -> Result<(), status::Custom<String>> {
    if find_user(&form.username).await.is_ok() {
        Err(status::Custom(
            Status::Conflict,
            "Username already exists".to_string(),
        ))
    } else {
        let user = User::new(&form.username, &form.password);
        store_user(user).await?;
        Ok(())
    }
}

async fn find_user(username: &str) -> Result<User, status::Custom<String>> {
    // Implementation to find user from the database
    unimplemented!()
}

async fn store_user(user: User) -> Result<(), status::Custom<String>> {
    // Implementation to store user in the database
    unimplemented!()
}