use rust_integration_services::http::{http_request::HttpRequest, http_response::HttpResponse};

// curl -i -H "Content-Type: application/json" -d '{"bucket":"my-bucket","key":"my-key"}' http://127.0.0.1:8080/xml
pub async fn json_to_xml(request: HttpRequest) -> HttpResponse {

    let body = request.body().as_bytes().await.unwrap();

    let json: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(json) => json,
        Err(_) => return HttpResponse::builder().status(400).body_bytes("Invalid JSON").unwrap(),
    };

    let xml = match quick_xml::se::to_string_with_root("Root", &json) {
        Ok(xml) => format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}", xml),
        Err(_) => return HttpResponse::builder().status(500).body_bytes("Could not parse XML").unwrap(),
    };

    HttpResponse::builder().status(200).body_bytes(xml).unwrap()
}