use hyper::body::to_bytes;
use hyper::header::{HeaderMap, COOKIE};
use hyper::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 创建 Hyper 客户端
    let client = Client::new();

    // 构建要发送的 JSON 数据
    let data = json!({
        "userName": "aba", //用户名称
        "password": "12345678", //用户密码
        "sex": "1" //0(未知) 1(男) 2(女)
    });

    let mut headers = HeaderMap::new();
    headers.insert("COOKIE", "666");

    // 将 JSON 数据转换为字节数组
    let body = serde_json::to_vec(&data)?;

    let request1 = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri("http://localhost:3000/login")
        .header("Content-Type", "application/json");

    // 构建 POST 请求
    let request = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri("http://localhost:3000/session")
        .header("Content-Type", "application/json")
        .body(hyper::Body::from(body))?;

    // 发送请求并等待响应
    let response = client.request(request).await?;

    // 打印响应状态码
    println!("Response status: {}", response.status());

    // 读取响应体并打印
    // 读取响应体并打印
    let body_bytes = to_bytes(response.into_body()).await?;
    println!("Response body: {:?}", body_bytes);

    Ok(())
}
