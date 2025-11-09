use axum::{Router, routing::post};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod api;
mod codegen;
mod config;
mod docker;
mod executor;
mod model;
mod parser;

use config::Config;

#[tokio::main]
async fn main() {
    // 加载配置
    let config = Config::load("config.toml").expect("配置加载失败");
    println!("📋 配置加载完成");

    // 初始化 Docker 客户端
    docker::client::init_docker(&config).expect("Docker 客户端初始化失败");

    // 打印启用的语言
    println!("\n📚 Enabled languages:");
    for (lang, cfg) in &config.languages {
        if cfg.enabled
            && let Some(container) = config.containers.get(lang)
        {
            println!(
                "  - {}: {} (container: {})",
                lang, cfg.file_extension, container
            );
        }
    }

    let config = Arc::new(config);
    let bind_addr = config.server.bind_addr.clone();

    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许所有来源
        .allow_methods(Any) // 允许所有方法（GET, POST 等）
        .allow_headers(Any); // 允许所有头部

    // 创建路由，传递配置
    let app = Router::new()
        .route(
            "/run",
            post({
                let config = Arc::clone(&config);
                move |body| api::run_code::run_code(Arc::clone(&config), body)
            }),
        )
        .layer(cors);

    println!("\n🚀 Server running on http://{}", bind_addr);
    println!("📦 Ready to execute code\n");

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind server");

    axum::serve(listener, app).await.expect("Server error");
}
