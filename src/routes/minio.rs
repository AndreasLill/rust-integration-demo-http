use rust_integration_services::{http::{http_request::HttpRequest, http_response::HttpResponse}, s3::{s3_client::S3Client, s3_client_config::S3ClientConfig}};

// curl -i -H "key: doc.txt" --data-binary @/home/andreas/file.txt http://127.0.0.1:8080/upload
pub async fn upload(request: HttpRequest) -> HttpResponse {

    let key = match request.headers().get("key") {
        Some(key) => key.to_owned(),
        None => return HttpResponse::builder().status(400).body_bytes("Missing header: key").unwrap(),
    };

    let body = request.body().as_bytes().await.unwrap();

    let config = S3ClientConfig::new("http://127.0.0.1:9000").access_key("minioadmin").secret_key("minioadmin");

    match S3Client::new(config).bucket("files").put_object_bytes(key.to_str().unwrap(), body).await {
        Ok(_) => HttpResponse::builder().status(200).body_empty().unwrap(),
        Err(err) => HttpResponse::builder().status(500).body_bytes(err.to_string()).unwrap(),
    }
}

// curl -i -H "key: doc.txt" http://127.0.0.1:8080/download
pub async fn download(request: HttpRequest) -> HttpResponse {

    let key = match request.headers().get("key") {
        Some(key) => key.to_owned(),
        None => return HttpResponse::builder().status(400).body_bytes("Missing header: key").unwrap(),
    };

    let config = S3ClientConfig::new("http://127.0.0.1:9000").access_key("minioadmin").secret_key("minioadmin");
    
    match S3Client::new(config).bucket("files").get_object_bytes(key.to_str().unwrap()).await {
        Ok(bytes) => HttpResponse::builder().status(200).body_bytes(bytes).unwrap(),
        Err(err) => HttpResponse::builder().status(500).body_bytes(err.to_string()).unwrap(),
    }
}