use rust_integration_services::{
    file::file_client::FileClient,
    http::{http_request::HttpRequest, http_response::HttpResponse},
};

pub async fn swagger(_: HttpRequest) -> HttpResponse {
    let bytes = FileClient::new()
        .read_from("swagger.html")
        .as_bytes()
        .await
        .unwrap();

    HttpResponse::builder()
        .status(200)
        .body_bytes(bytes)
        .unwrap()
}

pub async fn spec(_: HttpRequest) -> HttpResponse {
    let bytes = FileClient::new()
        .read_from("spec.yaml")
        .as_bytes()
        .await
        .unwrap();
    
    HttpResponse::builder()
        .status(200)
        .body_bytes(bytes)
        .unwrap()
}
