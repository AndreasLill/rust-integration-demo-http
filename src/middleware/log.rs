use rust_integration_services::http::{http_request::HttpRequest, http_response::HttpResponse, server::http_server::BeforeResult};

pub async fn log_request(request: HttpRequest) -> BeforeResult {
    tracing::info!(?request);
    BeforeResult::Next(request)
}

pub async fn log_response(response: HttpResponse) -> HttpResponse {
    tracing::info!(?response);
    response
}