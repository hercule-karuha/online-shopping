use serde::Deserialize;
#[derive(Deserialize)]
pub struct RegisterRequest {
    userName: String,
    password: String,
    sex: String,
}
