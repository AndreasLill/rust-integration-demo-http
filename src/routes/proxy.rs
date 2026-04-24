use rust_integration_services::http::{
    client::http_client::HttpClient, http_request::HttpRequest, http_response::HttpResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    results: Vec<Location>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    latitude: f64,
    longitude: f64,
    country: Option<String>,
}

pub async fn weather(request: HttpRequest) -> HttpResponse {
    let city = request.param("city").unwrap();
    let uri = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}",
        city
    );
    let req = HttpRequest::builder().get(uri).body_empty().unwrap();
    let response = match HttpClient::new().send(req).await {
        Ok(res) => res,
        Err(_) => {
            return HttpResponse::builder()
                .status(500)
                .body_bytes("Could not fetch geocoding data")
                .unwrap();
        }
    };

    let body = response.body().to_bytes().await.unwrap();
    let geocoding: GeocodingResponse = match serde_json::from_slice(&body) {
        Ok(res) => res,
        Err(_) => {
            return HttpResponse::builder()
                .status(400)
                .body_bytes("Could not parse weather data")
                .unwrap();
        }
    };
    let first = geocoding.results.first().unwrap();

    let uri = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        first.latitude, first.longitude
    );
    let req = HttpRequest::builder().get(uri).body_empty().unwrap();

    match HttpClient::new().send(req).await {
        Ok(res) => res,
        Err(_) => HttpResponse::builder()
            .status(500)
            .body_bytes("Could not fetch current weather data")
            .unwrap(),
    }
}
