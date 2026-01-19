use warp::Filter;
use serde::{Serialize, Deserialize};
use tokio;

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[tokio::main]
async fn main() {
    // 라우팅 설정: "/hello"로 들어오는 요청에 대해 JSON 응답 반환
    let hello = warp::path("hello")
        .map(|| warp::reply::json(&Response {
            message: String::from("Hello, world!"),
        }));

    // 서버 시작: 3030 포트에서 대기
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
