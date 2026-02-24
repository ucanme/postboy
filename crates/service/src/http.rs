//! HTTP service implementation

use reqwest::Client;
use reqwest::header::HeaderValue;
use anyhow::Result;
use std::time::Duration;

use postboy_models::{Request, Response, HttpMethod, ResponseHeader, ResponseBody};

/// HTTP service for making API requests
pub struct HttpService {
    client: Client,
}

impl HttpService {
    /// Create a new HTTP service
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { client }
    }

    /// Get the underlying reqwest client
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Send an HTTP request and return the response
    pub async fn send_request(&self, request: &Request) -> Result<Response> {
        let url = &request.url.raw;

        // Build the request based on method
        let mut req_builder = match request.method {
            HttpMethod::GET => self.client.get(url),
            HttpMethod::POST => self.client.post(url),
            HttpMethod::PUT => self.client.put(url),
            HttpMethod::DELETE => self.client.delete(url),
            HttpMethod::PATCH => self.client.patch(url),
            HttpMethod::HEAD => self.client.head(url),
            HttpMethod::OPTIONS => self.client.request(reqwest::Method::OPTIONS, url),
        };

        // Add headers
        for header in &request.headers {
            if header.enabled {
                if let Ok(name) = reqwest::header::HeaderName::from_bytes(header.key.as_bytes()) {
                    if let Ok(value) = HeaderValue::from_str(&header.value) {
                        req_builder = req_builder.header(name, value);
                    }
                }
            }
        }

        // Add query parameters
        if !request.query_params.is_empty() {
            let params: Vec<(String, String)> = request.query_params
                .iter()
                .filter(|p| p.enabled)
                .map(|p| (p.key.clone(), p.value.clone()))
                .collect();

            if !params.is_empty() {
                req_builder = req_builder.query(&params);
            }
        }

        // Add body if present
        match &request.body {
            postboy_models::RequestBody::None => {}
            postboy_models::RequestBody::Json { raw } => {
                req_builder = req_builder
                    .header("Content-Type", "application/json")
                    .body(raw.clone());
            }
            postboy_models::RequestBody::FormData { .. } => {
                // TODO: Implement form data encoding
            }
            postboy_models::RequestBody::UrlEncoded { .. } => {
                // TODO: Implement URL encoded form data
            }
            postboy_models::RequestBody::Raw { raw, language } => {
                if let Some(lang) = language {
                    req_builder = req_builder.header("Content-Type", lang.clone());
                }
                req_builder = req_builder.body(raw.clone());
            }
            postboy_models::RequestBody::Binary => {}
        }

        // Send the request
        let start = std::time::Instant::now();
        let http_response = req_builder.send().await?;
        let duration_ms = start.elapsed().as_millis() as u64;

        // Get status code
        let status_code = http_response.status().as_u16();

        // Get status text
        let status_text = http_response.status().canonical_reason()
            .unwrap_or("Unknown")
            .to_string();

        // Get response headers
        let response_headers: Vec<ResponseHeader> = http_response
            .headers()
            .iter()
            .map(|(name, value)| ResponseHeader {
                name: name.as_str().to_string(),
                value: value.to_str().unwrap_or("").to_string(),
            })
            .collect();

        // Determine content type before consuming response
        let content_type = http_response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("text/plain")
            .to_string();

        // Get response body
        let body_raw = http_response.text().await?;
        let body_bytes = body_raw.as_bytes().to_vec();

        // Calculate response size
        let size = body_bytes.len() as u64;

        // Try to parse as JSON, otherwise use text
        let response_body = if content_type.contains("json") {
            serde_json::from_str(&body_raw)
                .map(|v| ResponseBody::Json(v))
                .unwrap_or_else(|_| ResponseBody::Text(body_raw.clone()))
        } else if body_raw.is_empty() {
            ResponseBody::Empty
        } else {
            ResponseBody::Text(body_raw.clone())
        };

        // Get current timestamp for received_at
        let received_at = postboy_models::now();

        Ok(Response {
            status_code,
            status_text,
            headers: response_headers,
            body: response_body,
            duration_ms,
            size,
            cookies: Vec::new(),
            received_at,
            test_results: Vec::new(),
            errors: Vec::new(),
        })
    }

    /// Send a simple GET request
    pub async fn get(&self, url: &str) -> Result<Response> {
        let request = Request::new(
            "Simple GET".to_string(),
            HttpMethod::GET,
            url.to_string(),
        );
        self.send_request(&request).await
    }

    /// Send a simple POST request with JSON body
    pub async fn post_json(&self, url: &str, body: &str) -> Result<Response> {
        let request = Request::new(
            "Simple POST".to_string(),
            HttpMethod::POST,
            url.to_string(),
        )
        .with_body(postboy_models::RequestBody::json(body.to_string()));
        self.send_request(&request).await
    }
}

impl Default for HttpService {
    fn default() -> Self {
        Self::new()
    }
}
