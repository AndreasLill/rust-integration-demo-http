use rust_integration_services::http::{client::http_client::HttpClient, http_request::HttpRequest, http_response::HttpResponse};

// curl -i http://127.0.0.1:8080/proxy/helloworld
pub async fn httpbin(request: HttpRequest) -> HttpResponse {

    let value = request.params().get("value").unwrap();

    let uri = format!("https://httpbin.org/anything/{}", value);
    let req = HttpRequest::builder().get(uri).body_empty().unwrap();

    match HttpClient::new().send(req).await {
        Ok(response) => response,
        Err(err) => HttpResponse::builder().status(500).body_bytes(err.to_string()).unwrap(),
    }
}