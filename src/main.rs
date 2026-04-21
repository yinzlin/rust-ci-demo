use std::net::SocketAddr;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // 创建路由
    let app = Router::new().route("/", get(|| async { "Hello CI/CD89fasdf!" }));

    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server running on http://{}", addr);

    // 创建TCP监听器
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind port");

    // 启动服务器
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
