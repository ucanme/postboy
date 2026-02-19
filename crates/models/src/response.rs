//! HTTP response model

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::{Timestamp, now};

/// HTTP response from a request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// HTTP status code
    pub status_code: u16,

    /// HTTP status text (e.g., "OK", "Not Found")
    pub status_text: String,

    /// Response headers
    pub headers: Vec<ResponseHeader>,

    /// Response body
    pub body: ResponseBody,

    /// Request duration in milliseconds
    pub duration_ms: u64,

    /// Response size in bytes
    pub size: u64,

    /// Cookie values received
    #[serde(default)]
    pub cookies: Vec<Cookie>,

    /// Timestamp when response was received
    pub received_at: Timestamp,

    /// Test results from post-response scripts
    #[serde(default)]
    pub test_results: Vec<TestResult>,

    /// Any errors that occurred during the request
    #[serde(default)]
    pub errors: Vec<ResponseError>,
}

impl Response {
    /// Create a new response
    pub fn new(status_code: u16, status_text: String) -> Self {
        Self {
            status_code,
            status_text,
            headers: Vec::new(),
            body: ResponseBody::Empty,
            duration_ms: 0,
            size: 0,
            cookies: Vec::new(),
            received_at: now(),
            test_results: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Create an error response
    pub fn error(message: String) -> Self {
        Self {
            status_code: 0,
            status_text: "Error".to_string(),
            headers: Vec::new(),
            body: ResponseBody::Text(message),
            duration_ms: 0,
            size: 0,
            cookies: Vec::new(),
            received_at: now(),
            test_results: Vec::new(),
            errors: vec![ResponseError {
                code: "REQUEST_ERROR".to_string(),
                message,
                stack: None,
            }],
        }
    }

    /// Check if the response was successful (2xx status code)
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status_code)
    }

    /// Check if the response was a redirect (3xx status code)
    pub fn is_redirect(&self) -> bool {
        (300..400).contains(&self.status_code)
    }

    /// Check if the response was a client error (4xx status code)
    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status_code)
    }

    /// Check if the response was a server error (5xx status code)
    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status_code)
    }

    /// Get the content type from headers
    pub fn content_type(&self) -> Option<String> {
        self.headers
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case("content-type"))
            .and_then(|h| h.value.split(';').next())
            .map(String::from)
    }

    /// Get a header value by name (case-insensitive)
    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(name))
            .map(|h| &h.value)
    }

    /// Parse response body as JSON
    pub fn json(&self) -> Result<serde_json::Value, JsonError> {
        match &self.body {
            ResponseBody::Json(value) => Ok(value.clone()),
            ResponseBody::Text(text) => {
                serde_json::from_str(text).map_err(|e| JsonError::Parse(e.to_string()))
            }
            ResponseBody::Empty => Err(JsonError::Empty),
            _ => Err(JsonError::NotJson),
        }
    }

    /// Get response body as text
    pub fn text(&self) -> String {
        match &self.body {
            ResponseBody::Text(text) => text.clone(),
            ResponseBody::Json(value) => value.to_string(),
            ResponseBody::Empty => String::new(),
            ResponseBody::Binary(data) => String::from_utf8_lossy(data).to_string(),
        }
    }

    /// Get response body as bytes
    pub fn bytes(&self) -> Vec<u8> {
        match &self.body {
            ResponseBody::Text(text) => text.as_bytes().to_vec(),
            ResponseBody::Json(value) => value.to_string().as_bytes().to_vec(),
            ResponseBody::Empty => Vec::new(),
            ResponseBody::Binary(data) => data.clone(),
        }
    }

    /// Get formatted duration string
    pub fn duration_str(&self) -> String {
        format_duration(self.duration_ms)
    }

    /// Get formatted size string
    pub fn size_str(&self) -> String {
        format_bytes(self.size)
    }

    /// Add a test result
    pub fn add_test_result(&mut self, result: TestResult) {
        self.test_results.push(result);
    }

    /// Get all passed tests
    pub fn passed_tests(&self) -> Vec<&TestResult> {
        self.test_results
            .iter()
            .filter(|t| t.passed)
            .collect()
    }

    /// Get all failed tests
    pub fn failed_tests(&self) -> Vec<&TestResult> {
        self.test_results
            .iter()
            .filter(|t| !t.passed)
            .collect()
    }

    /// Check if all tests passed
    pub fn all_tests_passed(&self) -> bool {
        self.test_results.iter().all(|t| t.passed)
    }
}

/// Response header
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseHeader {
    pub name: String,
    pub value: String,
}

impl ResponseHeader {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

/// Response body types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ResponseBody {
    Empty,
    Text(String),
    Json(serde_json::Value),
    Binary(Vec<u8>),
}

impl ResponseBody {
    pub fn is_empty(&self) -> bool {
        matches!(self, ResponseBody::Empty)
    }

    pub fn len(&self) -> usize {
        match self {
            ResponseBody::Empty => 0,
            ResponseBody::Text(s) => s.len(),
            ResponseBody::Json(v) => v.to_string().len(),
            ResponseBody::Binary(b) => b.len(),
        }
    }
}

/// Cookie received in response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expires: Option<Timestamp>,
    pub http_only: bool,
    pub secure: bool,
    pub same_site: Option<SameSite>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

/// Test result from post-response script
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestResult {
    /// Test name/description
    pub name: String,
    /// Whether the test passed
    pub passed: bool,
    /// Error message if test failed
    pub error_message: Option<String>,
    /// Duration of test in milliseconds
    pub duration_ms: Option<u64>,
}

impl TestResult {
    /// Create a passed test result
    pub fn passed(name: String) -> Self {
        Self {
            name,
            passed: true,
            error_message: None,
            duration_ms: None,
        }
    }

    /// Create a failed test result
    pub fn failed(name: String, error_message: String) -> Self {
        Self {
            name,
            passed: false,
            error_message: Some(error_message),
            duration_ms: None,
        }
    }
}

/// Response error
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseError {
    pub code: String,
    pub message: String,
    pub stack: Option<String>,
}

/// JSON parsing error
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum JsonError {
    #[error("Response body is empty")]
    Empty,
    #[error("Response is not JSON")]
    NotJson,
    #[error("Failed to parse JSON: {0}")]
    Parse(String),
}

/// Format duration in human-readable form
pub fn format_duration(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        let minutes = ms / 60000;
        let seconds = (ms % 60000) / 1000;
        format!("{}m {}s", minutes, seconds)
    }
}

/// Format bytes in human-readable form
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes < KB {
        format!("{}B", bytes)
    } else if bytes < MB {
        format!("{:.2}KB", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.2}MB", bytes as f64 / MB as f64)
    } else {
        format!("{:.2}GB", bytes as f64 / GB as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_creation() {
        let response = Response::new(200, "OK".to_string());
        assert_eq!(response.status_code, 200);
        assert_eq!(response.status_text, "OK");
        assert!(response.is_success());
        assert!(!response.is_redirect());
        assert!(!response.is_client_error());
        assert!(!response.is_server_error());
    }

    #[test]
    fn test_response_categories() {
        assert!(Response::new(200, "OK".to_string()).is_success());
        assert!(Response::new(201, "Created".to_string()).is_success());
        assert!(Response::new(204, "No Content".to_string()).is_success());

        assert!(Response::new(301, "Moved Permanently".to_string()).is_redirect());
        assert!(Response::new(302, "Found".to_string()).is_redirect());

        assert!(Response::new(400, "Bad Request".to_string()).is_client_error());
        assert!(Response::new(404, "Not Found".to_string()).is_client_error());

        assert!(Response::new(500, "Internal Server Error".to_string()).is_server_error());
        assert!(Response::new(503, "Service Unavailable".to_string()).is_server_error());
    }

    #[test]
    fn test_response_json() {
        let json_value = serde_json::json!({"message": "hello"});
        let response = Response {
            body: ResponseBody::Json(json_value.clone()),
            ..Response::new(200, "OK".to_string())
        };

        let parsed = response.json().unwrap();
        assert_eq!(parsed, json_value);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(100), "100ms");
        assert_eq!(format_duration(1500), "1.5s");
        assert_eq!(format_duration(65000), "1m 5s");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(100), "100B");
        assert_eq!(format_bytes(2048), "2.00KB");
        assert_eq!(format_bytes(3_145_728), "3.00MB");
        assert_eq!(format_bytes(1_073_741_824), "1.00GB");
    }

    #[test]
    fn test_test_results() {
        let mut response = Response::new(200, "OK".to_string());

        response.add_test_result(TestResult::passed("Status is 200".to_string()));
        response.add_test_result(TestResult::failed("Has data".to_string(), "No data found".to_string()));
        response.add_test_result(TestResult::passed("Response time OK".to_string()));

        assert_eq!(response.test_results.len(), 3);
        assert_eq!(response.passed_tests().len(), 2);
        assert_eq!(response.failed_tests().len(), 1);
        assert!(!response.all_tests_passed());
    }
}
