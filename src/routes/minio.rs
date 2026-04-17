use rust_integration_services::{http::{http_request::HttpRequest, http_response::HttpResponse}, s3::{s3_client::S3Client, s3_client_config::S3ClientConfig}};

// curl -i -H "Bucket: files" -H "Key: file.txt" --data-binary @/home/andreas/file.txt http://127.0.0.1:8080/upload
pub async fn upload(request: HttpRequest) -> HttpResponse {

    let bucket = match request.headers().get("Bucket") {
        Some(bucket) => bucket.to_str().unwrap(),
        None => return HttpResponse::builder().status(400).body_bytes("Missing header: Bucket").unwrap(),
    };

    let key = match request.headers().get("Key") {
        Some(key) => key.to_str().unwrap(),
        None => return HttpResponse::builder().status(400).body_bytes("Missing header: Key").unwrap(),
    };

    let config = S3ClientConfig::builder()
    .endpoint("http://127.0.0.1:9000")
    .access_key("minioadmin")
    .secret_key("minioadmin")
    .build()
    .unwrap();

    let response = match S3Client::new(config).bucket(bucket).put_object(key).from_stream(request.body()).await {
        Ok(_) => HttpResponse::builder().status(200).body_empty().unwrap(),
        Err(err) => HttpResponse::builder().status(500).body_bytes(err.to_string()).unwrap(),
    };

    response
}

// curl -i -H "Bucket: files" -H "Key: file.txt" http://127.0.0.1:8080/download
pub async fn download(request: HttpRequest) -> HttpResponse {

    let bucket = match request.headers().get("Bucket") {
        Some(bucket) => bucket.to_str().unwrap(),
        None => return HttpResponse::builder().status(400).body_bytes("Missing header: Bucket").unwrap(),
    };

    let key = match request.headers().get("Key") {
        Some(key) => key.to_str().unwrap(),
        None => return HttpResponse::builder().status(400).body_bytes("Missing header: Key").unwrap(),
    };

    let config = S3ClientConfig::builder()
    .endpoint("http://127.0.0.1:9000")
    .access_key("minioadmin")
    .secret_key("minioadmin")
    .build()
    .unwrap();
    
    let response = match S3Client::new(config).bucket(bucket).get_object(key).as_stream().await {
        Ok(stream) => HttpResponse::builder().status(200).body_stream(stream).unwrap(),
        Err(err) => HttpResponse::builder().status(500).body_bytes(err.to_string()).unwrap(),
    };

    response
}