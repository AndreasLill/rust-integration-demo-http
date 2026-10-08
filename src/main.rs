use rust_integration_services::http::server::{
    http_server::HttpServer, http_server_config::HttpServerConfig,
};

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    HttpServer::builder(HttpServerConfig::new("0.0.0.0", 8080))
        .route("/xml", routes::transform::json_to_xml)
        .route("/weather/{city}", routes::proxy::weather)
        .route("/upload", routes::minio::upload)
        .route("/download", routes::minio::download)
        .route("/spec.yaml", routes::openapi::spec)
        .route("/", routes::openapi::swagger)
        .before(async move |req| {
            tracing::info!("{} {}", req.method(), req.path().to_string());
            req
        })
        .after(async move |res| {
            tracing::info!("{}", res.status());
            res
        })
        .build()
        .run()
        .await;
}
