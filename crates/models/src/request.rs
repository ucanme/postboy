//! HTTP request model

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::{Id, Timestamp, new_id, now, Temporal, Identifiable};

/// HTTP request method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl HttpMethod {
    pub const ALL: [HttpMethod; 7] = [
        HttpMethod::GET,
        HttpMethod::POST,
        HttpMethod::PUT,
        HttpMethod::DELETE,
        HttpMethod::PATCH,
        HttpMethod::HEAD,
        HttpMethod::OPTIONS,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            _ => Err(format!("Invalid HTTP method: {}", s)),
        }
    }
}

/// HTTP header
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

impl Header {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            enabled: true,
        }
    }

    pub fn disabled(key: String, value: String) -> Self {
        Self {
            key,
            value,
            enabled: false,
        }
    }
}

/// Query parameter
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Param {
    pub key: String,
    pub value: String,
    pub enabled: bool,
    pub description: Option<String>,
}

impl Param {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            enabled: true,
            description: None,
        }
    }
}

/// Form data field
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormField {
    pub key: String,
    pub value: String,
    pub enabled: bool,
    pub file: Option<FileField>,
}

impl FormField {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            enabled: true,
            file: None,
        }
    }

    pub fn file(key: String, file: FileField) -> Self {
        Self {
            key,
            value: String::new(),
            enabled: true,
            file: Some(file),
        }
    }
}

/// File field for multipart uploads
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileField {
    pub name: String,
    pub content_type: Option<String>,
    pub path: Option<String>,
}

impl FileField {
    pub fn new(name: String) -> Self {
        Self {
            name,
            content_type: None,
            path: None,
        }
    }

    pub fn with_content_type(mut self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }
}

/// Request body types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum RequestBody {
    None,
    Json { raw: String },
    FormData { formdata: Vec<FormField> },
    UrlEncoded { urlencoded: Vec<FormField> },
    Raw { raw: String, language: Option<String> },
    Binary,
}

impl RequestBody {
    pub fn none() -> Self {
        Self::None
    }

    pub fn json(raw: String) -> Self {
        Self::Json { raw }
    }

    pub fn form_data(formdata: Vec<FormField>) -> Self {
        Self::FormData { formdata }
    }

    pub fn url_encoded(urlencoded: Vec<FormField>) -> Self {
        Self::UrlEncoded { urlencoded }
    }

    pub fn raw(raw: String) -> Self {
        Self::Raw {
            raw,
            language: None,
        }
    }

    pub fn raw_with_language(raw: String, language: String) -> Self {
        Self::Raw {
            raw,
            language: Some(language),
        }
    }

    pub fn binary() -> Self {
        Self::Binary
    }

    pub fn mode(&self) -> BodyMode {
        match self {
            RequestBody::None => BodyMode::None,
            RequestBody::Json { .. } => BodyMode::Json,
            RequestBody::FormData { .. } => BodyMode::FormData,
            RequestBody::UrlEncoded { .. } => BodyMode::UrlEncoded,
            RequestBody::Raw { .. } => BodyMode::Raw,
            RequestBody::Binary => BodyMode::Binary,
        }
    }

    pub fn get_raw(&self) -> Option<&str> {
        match self {
            RequestBody::None => None,
            RequestBody::Json { raw } => Some(raw),
            RequestBody::FormData { .. } => None,
            RequestBody::UrlEncoded { .. } => None,
            RequestBody::Raw { raw, .. } => Some(raw),
            RequestBody::Binary => None,
        }
    }

    pub fn get_json(&self) -> Option<&serde_json::Value> {
        match self {
            RequestBody::Json { raw } => serde_json::from_str(raw).ok(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BodyMode {
    None,
    Json,
    FormData,
    UrlEncoded,
    Raw,
    Binary,
}

impl BodyMode {
    pub const ALL: [BodyMode; 6] = [
        BodyMode::None,
        BodyMode::Json,
        BodyMode::FormData,
        BodyMode::UrlEncoded,
        BodyMode::Raw,
        BodyMode::Binary,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            BodyMode::None => "none",
            BodyMode::Json => "json",
            BodyMode::FormData => "formdata",
            BodyMode::UrlEncoded => "urlencoded",
            BodyMode::Raw => "raw",
            BodyMode::Binary => "binary",
        }
    }
}

/// Authentication configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AuthConfig {
    Noauth,
    Bearer { token: String },
    Basic {
        username: String,
        password: String,
    },
    ApiKey {
        key: String,
        value: String,
        #[serde(default)]
        add_to: ApiKeyLocation,
    },
    Digest {
        username: String,
        password: String,
    },
    OAuth1 {
        consumer_key: String,
        consumer_secret: String,
        token: String,
        token_secret: String,
        signature_method: String,
        timestamp: Option<String>,
        nonce: Option<String>,
        version: Option<String>,
        realm: Option<String>,
    },
    OAuth2 {
        config: OAuth2Config,
    },
    Awsv4 {
        access_key: String,
        secret_key: String,
        region: String,
        service: String,
    },
    Hawk {
        auth_id: String,
        auth_key: String,
        algorithm: String,
        user: String,
        nonce: Option<String>,
        ext: Option<String>,
        mac: Option<String>,
        timestamp: Option<String>,
    },
    BearerCustom {
        #[serde(flatten)]
        config: HashMap<String, serde_json::Value>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyLocation {
    Header,
    Query,
}

impl Default for ApiKeyLocation {
    fn default() -> Self {
        ApiKeyLocation::Header
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub scope: String,
    pub redirect_url: String,
    pub auth_url: String,
    pub access_token_url: String,
    pub grant_type: String,
}

/// Script configuration for request hooks
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ScriptConfig {
    /// Pre-request script (executed before sending the request)
    pub pre_request: Option<String>,
    /// Post-response script (executed after receiving the response)
    pub post_response: Option<String>,
    /// Test script (assertions)
    pub test: Option<String>,
}

/// HTTP request model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,

    /// HTTP method
    #[serde(rename = "method")]
    pub method: HttpMethod,

    /// Request URL (may contain variables like {{base_url}})
    pub url: Url,

    /// HTTP headers
    #[serde(default)]
    pub headers: Vec<Header>,

    /// Query parameters
    #[serde(default)]
    pub query_params: Vec<Param>,

    /// Request body
    #[serde(default)]
    pub body: RequestBody,

    /// Authentication configuration
    pub auth: Option<AuthConfig>,

    /// Script hooks
    #[serde(default)]
    pub script: ScriptConfig,

    /// Parent collection ID
    pub collection_id: Option<Id>,

    /// Parent folder ID
    pub folder_id: Option<Id>,

    /// Creation timestamp
    pub created_at: Timestamp,

    /// Last update timestamp
    pub updated_at: Timestamp,

    /// UI-specific state
    #[serde(default)]
    pub ui_state: RequestUiState,
}

/// URL representation that preserves the raw string
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Url {
    /// Raw URL string (may contain variables)
    pub raw: String,
    /// Protocol (http, https, etc.)
    pub protocol: Option<String>,
    /// Host
    pub host: Option<String>,
    /// Port
    pub port: Option<u16>,
    /// Path
    pub path: Option<String>,
    /// Query string
    pub query: Option<String>,
    /// Hash fragment
    pub hash: Option<String>,
}

impl Url {
    pub fn new(raw: String) -> Self {
        Self {
            raw,
            protocol: None,
            host: None,
            port: None,
            path: None,
            query: None,
            hash: None,
        }
    }

    pub fn parse(raw: String) -> Result<Self, String> {
        let parsed = url::Url::parse(&raw).map_err(|e| e.to_string())?;

        Ok(Self {
            raw,
            protocol: parsed.scheme().to_string().into(),
            host: parsed.host_str().map(String::from),
            port: parsed.port(),
            path: Some(parsed.path().to_string()),
            query: parsed.query().map(String::from),
            hash: parsed.fragment().map(String::from),
        })
    }
}

/// UI-specific state for requests
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestUiState {
    /// Whether the request is expanded in the sidebar
    pub is_expanded: bool,
    /// Selected tab index
    pub selected_tab: u32,
    /// Scroll position
    pub scroll_position: Option<f32>,
}

impl Request {
    pub fn new(name: String, method: HttpMethod, url: String) -> Self {
        let now = now();
        Self {
            id: new_id(),
            name,
            description: None,
            method,
            url: Url::new(url),
            headers: Vec::new(),
            query_params: Vec::new(),
            body: RequestBody::none(),
            auth: None,
            script: ScriptConfig::default(),
            collection_id: None,
            folder_id: None,
            created_at: now,
            updated_at: now,
            ui_state: RequestUiState::default(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_collection(mut self, collection_id: Id) -> Self {
        self.collection_id = Some(collection_id);
        self
    }

    pub fn with_folder(mut self, folder_id: Id) -> Self {
        self.folder_id = Some(folder_id);
        self
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.push(Header::new(key, value));
        self
    }

    pub fn with_query_param(mut self, key: String, value: String) -> Self {
        self.query_params.push(Param::new(key, value));
        self
    }

    pub fn with_body(mut self, body: RequestBody) -> Self {
        self.body = body;
        self
    }

    pub fn with_auth(mut self, auth: AuthConfig) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn with_pre_request_script(mut self, script: String) -> Self {
        self.script.pre_request = Some(script);
        self
    }

    pub fn with_post_response_script(mut self, script: String) -> Self {
        self.script.post_response = Some(script);
        self
    }

    pub fn with_test_script(mut self, script: String) -> Self {
        self.script.test = Some(script);
        self
    }

    /// Get all enabled headers
    pub fn enabled_headers(&self) -> Vec<&Header> {
        self.headers.iter().filter(|h| h.enabled).collect()
    }

    /// Get all enabled query parameters
    pub fn enabled_query_params(&self) -> Vec<&Param> {
        self.query_params.iter().filter(|p| p.enabled).collect()
    }

    /// Check if request has a body
    pub fn has_body(&self) -> bool {
        !matches!(self.body, RequestBody::None)
    }

    /// Create a duplicate of this request with a new ID
    pub fn duplicate(&self) -> Self {
        let mut dup = self.clone();
        dup.id = new_id();
        dup.name = format!("{} (Copy)", dup.name);
        dup.created_at = now();
        dup.updated_at = now();
        dup
    }
}

impl Temporal for Request {
    fn created_at(&self) -> Timestamp {
        self.created_at
    }

    fn updated_at(&self) -> Timestamp {
        self.updated_at
    }
}

impl Identifiable for Request {
    fn id(&self) -> Id {
        self.id
    }
}

/// Builder pattern for creating requests
pub struct RequestBuilder {
    request: Request,
}

impl RequestBuilder {
    pub fn new(name: String, method: HttpMethod, url: String) -> Self {
        Self {
            request: Request::new(name, method, url),
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.request.description = Some(description);
        self
    }

    pub fn header(mut self, key: String, value: String) -> Self {
        self.request.headers.push(Header::new(key, value));
        self
    }

    pub fn headers(mut self, headers: Vec<Header>) -> Self {
        self.request.headers = headers;
        self
    }

    pub fn query_param(mut self, key: String, value: String) -> Self {
        self.request.query_params.push(Param::new(key, value));
        self
    }

    pub fn body(mut self, body: RequestBody) -> Self {
        self.request.body = body;
        self
    }

    pub fn auth(mut self, auth: AuthConfig) -> Self {
        self.request.auth = Some(auth);
        self
    }

    pub fn collection(mut self, collection_id: Id) -> Self {
        self.request.collection_id = Some(collection_id);
        self
    }

    pub fn folder(mut self, folder_id: Id) -> Self {
        self.request.folder_id = Some(folder_id);
        self
    }

    pub fn pre_request_script(mut self, script: String) -> Self {
        self.request.script.pre_request = Some(script);
        self
    }

    pub fn post_response_script(mut self, script: String) -> Self {
        self.request.script.post_response = Some(script);
        self
    }

    pub fn test_script(mut self, script: String) -> Self {
        self.request.script.test = Some(script);
        self
    }

    pub fn build(self) -> Request {
        self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_creation() {
        let request = Request::new(
            "Test API".to_string(),
            HttpMethod::GET,
            "https://api.example.com/users".to_string(),
        );

        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.url.raw, "https://api.example.com/users");
        assert_eq!(request.headers.len(), 0);
    }

    #[test]
    fn test_request_builder() {
        let request = RequestBuilder::new(
            "Test API".to_string(),
            HttpMethod::POST,
            "https://api.example.com/users".to_string(),
        )
        .header("Content-Type".to_string(), "application/json".to_string())
        .body(RequestBody::json(r#"{"name":"John"}"#.to_string()))
        .build();

        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(request.headers.len(), 1);
        assert!(matches!(request.body, RequestBody::Json { .. }));
    }

    #[test]
    fn test_http_method_from_str() {
        assert_eq!(HttpMethod::from_str("GET"), Ok(HttpMethod::GET));
        assert_eq!(HttpMethod::from_str("get"), Ok(HttpMethod::GET));
        assert_eq!(HttpMethod::from_str("POST"), Ok(HttpMethod::POST));
        assert!(HttpMethod::from_str("INVALID").is_err());
    }

    #[test]
    fn test_enabled_headers() {
        let request = Request::new(
            "Test".to_string(),
            HttpMethod::GET,
            "https://example.com".to_string(),
        )
        .with_header("Accept".to_string(), "application/json".to_string())
        .with_header("X-Disabled".to_string(), "value".to_string());

        // Disable the second header
        request.headers[1].enabled = false;

        let enabled = request.enabled_headers();
        assert_eq!(enabled.len(), 1);
        assert_eq!(enabled[0].key, "Accept");
    }

    #[test]
    fn test_request_duplicate() {
        let original = Request::new(
            "Original".to_string(),
            HttpMethod::GET,
            "https://example.com".to_string(),
        );
        let copy = original.duplicate();

        assert_ne!(original.id, copy.id);
        assert_eq!(copy.name, "Original (Copy)");
        assert_eq!(copy.method, original.method);
        assert_eq!(copy.url.raw, original.url.raw);
    }
}
