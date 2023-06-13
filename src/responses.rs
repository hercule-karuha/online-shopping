use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    code: u32,
    msg: String,
    data: T,
}
#[derive(Debug, Deserialize)]
pub struct UserId {
    userId: String,
}
