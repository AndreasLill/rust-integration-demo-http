use rust_integration_services::{http::{client::http_client::HttpClient, http_request::HttpRequest, http_response::HttpResponse, server::{http_server::HttpServer, http_server_config::HttpServerConfig}}, s3::{s3_client::S3Client, s3_client_config::S3ClientConfig}};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // Create HTTP server configuration listening on 0.0.0.0:8080 with TLS enabled.
    let config = HttpServerConfig::new("0.0.0.0", 8080);

    // Run the http server with the configuration.
    HttpServer::new(config)
    .route("/xml", xml)
    .route("/proxy/{value}", proxy)
    .route("/upload", upload)
    .route("/download", download)
    .run()
    .await;
}

// curl -i -H "Content-Type: application/json" -d '{"bucket":"my-bucket","key":"my-key"}' http://127.0.0.1:8080/xml
async fn xml(request: HttpRequest) -> HttpResponse {

    // Read the body as bytes into memory.
    let body = match request.body_as_bytes().await {
        Ok(body) => body,
        Err(err) => return HttpResponse::builder().status(400).body_bytes(err.to_string()).build().unwrap(),
    };

    // Parse and deserialize body bytes as JSON into Document struct.
    let json: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(json) => json,
        Err(_) => return HttpResponse::builder().status(400).body_bytes("Invalid JSON").build().unwrap(),
    };

    // Parse and serialize Document struct into XML string.
    let xml = match quick_xml::se::to_string_with_root("Root", &json) {
        Ok(xml) => format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}", xml),
        Err(_) => return HttpResponse::builder().status(500).body_bytes("Could not parse XML").build().unwrap(),
    };

    // Build http response with XML string as body.
    HttpResponse::builder().status(200).body_bytes(xml).build().unwrap()
}

// curl -i http://127.0.0.1:8080/proxy/helloworld
async fn proxy(request: HttpRequest) -> HttpResponse {

    // Get the {value} parameter from path.
    let value = match request.params().get("value") {
        Some(value) => value,
        None => return HttpResponse::builder().status(400).body_bytes("Missing 'value' parameter").build().unwrap(),
    };

    // Format canonical uri with value.
    let uri = format!("https://httpbin.org/anything/{}", value);

    // Create a new request with the canonical uri.
    let req = HttpRequest::builder().uri(uri).method("GET").build().unwrap();

    // Send the request and handle response.
    let response = match HttpClient::new().send(req).await {
        Ok(response) => response,
        Err(err) => HttpResponse::builder().status(500).body_bytes(err.to_string()).build().unwrap(),
    };
    
    response
}

// curl -i -H "key: doc.txt" --data-binary @/home/andreas/file.txt http://127.0.0.1:8080/upload
async fn upload(request: HttpRequest) -> HttpResponse {

    let key = match request.headers().get("key") {
        Some(key) => key.to_owned(),
        None => return HttpResponse::builder().status(400).body_bytes("Missing 'key' header").build().unwrap(),
    };

    let body = match request.body_as_bytes().await {
        Ok(body) => body,
        Err(err) => return HttpResponse::builder().status(400).body_bytes(err.to_string()).build().unwrap(),
    };

    let config = S3ClientConfig::new("http://127.0.0.1:9000").access_key("minioadmin").secret_key("minioadmin");

    match S3Client::new(config).bucket("docs").put_object_bytes(key.to_str().unwrap(), body).await {
        Ok(_) => HttpResponse::builder().status(200).build().unwrap(),
        Err(_) => HttpResponse::builder().status(500).build().unwrap(),
    }
}

// curl -i -H "key: doc.txt" http://127.0.0.1:8080/download
async fn download(request: HttpRequest) -> HttpResponse {

    let key = match request.headers().get("key") {
        Some(key) => key.to_owned(),
        None => return HttpResponse::builder().status(400).body_bytes("Missing 'key' header").build().unwrap(),
    };

    let config = S3ClientConfig::new("http://127.0.0.1:9000").access_key("minioadmin").secret_key("minioadmin");

    match S3Client::new(config).bucket("docs").get_object_bytes(key.to_str().unwrap()).await {
        Ok(bytes) => HttpResponse::builder().status(200).body_bytes(bytes).build().unwrap(),
        Err(err) => HttpResponse::builder().status(500).body_bytes(err.to_string()).build().unwrap(),
    }
}