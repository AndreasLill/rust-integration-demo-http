use rust_integration_services::{http::{client::http_client::HttpClient, http_request::HttpRequest, http_response::HttpResponse, server::{http_server::HttpServer, http_server_config::HttpServerConfig}}, s3::{s3_client::S3Client, s3_client_config::S3ClientConfig}};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // Run the http server with the configuration.
    HttpServer::new(HttpServerConfig::new("0.0.0.0", 8080))
    .route("/xml", json_to_xml)
    .route("/proxy/{value}", httpbin_proxy)
    .route("/upload", upload)
    .route("/download", download)
    .run()
    .await;
}

// curl -i -H "Content-Type: application/json" -d '{"bucket":"my-bucket","key":"my-key"}' http://127.0.0.1:8080/xml
async fn json_to_xml(request: HttpRequest) -> HttpResponse {

    // Read the body as bytes into memory.
    let body = request.body_as_bytes().await.unwrap();

    // Parse and deserialize body bytes as JSON into Document struct.
    let json: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(json) => json,
        Err(_) => return HttpResponse::builder().status(400).body_bytes("Invalid JSON").unwrap(),
    };

    // Parse and serialize Document struct into XML string.
    let xml = match quick_xml::se::to_string_with_root("Root", &json) {
        Ok(xml) => format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}", xml),
        Err(_) => return HttpResponse::builder().status(500).body_bytes("Could not parse XML").unwrap(),
    };

    // Build http response with XML string as body.
    HttpResponse::builder().status(200).body_bytes(xml).unwrap()
}

// curl -i http://127.0.0.1:8080/proxy/helloworld
async fn httpbin_proxy(request: HttpRequest) -> HttpResponse {

    // Get the {value} parameter from path.
    let value = match request.params().get("value") {
        Some(value) => value,
        None => return HttpResponse::builder().status(400).body_bytes("Missing 'value' parameter").unwrap(),
    };

    // Format canonical uri with value.
    let uri = format!("https://httpbin.org/anything/{}", value);

    // Create a new request with the canonical uri.
    let req = HttpRequest::builder().uri(uri).method("GET").body_empty().unwrap();

    // Send the request and handle response.
    let response = match HttpClient::new().send(req).await {
        Ok(response) => response,
        Err(err) => HttpResponse::builder().status(500).body_bytes(err.to_string()).unwrap(),
    };
    
    response
}

// curl -i -H "key: doc.txt" --data-binary @/home/andreas/file.txt http://127.0.0.1:8080/upload
async fn upload(request: HttpRequest) -> HttpResponse {

    // Get 'key' header.
    let key = match request.headers().get("key") {
        Some(key) => key.to_owned(),
        None => return HttpResponse::builder().status(400).body_bytes("Missing header: key").unwrap(),
    };

    // Read the body as bytes into memory.
    let body = request.body_as_bytes().await.unwrap();

    // Create S3 client config for use with minio.
    let config = S3ClientConfig::new("http://127.0.0.1:9000").access_key("minioadmin").secret_key("minioadmin");

    // Put object as bytes using the S3 client.
    match S3Client::new(config).bucket("docs").put_object_bytes(key.to_str().unwrap(), body).await {
        Ok(_) => HttpResponse::builder().status(200).body_empty().unwrap(),
        Err(err) => HttpResponse::builder().status(500).body_bytes(err.to_string()).unwrap(),
    }
}

// curl -i -H "key: doc.txt" http://127.0.0.1:8080/download
async fn download(request: HttpRequest) -> HttpResponse {

    // Get 'key' header.
    let key = match request.headers().get("key") {
        Some(key) => key.to_owned(),
        None => return HttpResponse::builder().status(400).body_bytes("Missing header: key").unwrap(),
    };

    // Create S3 client config for use with minio.
    let config = S3ClientConfig::new("http://127.0.0.1:9000").access_key("minioadmin").secret_key("minioadmin");
    
    // Put object as bytes using the S3 client.
    match S3Client::new(config).bucket("docs").get_object_bytes(key.to_str().unwrap()).await {
        Ok(bytes) => HttpResponse::builder().status(200).body_bytes(bytes).unwrap(),
        Err(err) => HttpResponse::builder().status(500).body_bytes(err.to_string()).unwrap(),
    }
}