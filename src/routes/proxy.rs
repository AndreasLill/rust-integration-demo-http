use rust_integration_services::http::{client::http_client::HttpClient, http_request::HttpRequest, http_response::HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    results: Vec<Location>,
}

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
    country: Option<String>,
}

pub async fn weather(request: HttpRequest) -> HttpResponse {

    let city = request.params().get("city").unwrap();
    tracing::info!("city: {:?}", city);

    let uri = format!("https://geocoding-api.open-meteo.com/v1/search?name={}", city);
    let req = HttpRequest::builder().get(uri).body_empty().unwrap();
    let response = match HttpClient::new().send(req).await {
        Ok(response) => response,
        Err(err) => return HttpResponse::builder().status(500).body_bytes(err.to_string()).unwrap(),
    };
    tracing::info!("geocoding api response: {:?}", response);

    let body = &response.body().as_bytes().await.unwrap();
    let geocoding: GeocodingResponse = match serde_json::from_slice(&body) {
        Ok(json) => json,
        Err(_) => return HttpResponse::builder().status(400).body_bytes("Could not parse geocoding data").unwrap(),
    };

    let first = geocoding.results.get(0).unwrap();
    let uri = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true", first.latitude, first.longitude);
    let req = HttpRequest::builder().get(uri).body_empty().unwrap();
    let response = match HttpClient::new().send(req).await {
        Ok(response) => response,
        Err(err) => HttpResponse::builder().status(500).body_bytes(err.to_string()).unwrap(),
    };
    tracing::info!("weather api response: {:?}", response);

    response
}

