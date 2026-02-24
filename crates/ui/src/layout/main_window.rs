//! Main window component for Postboy - Interactive version

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{button::*, input::*, *};
use postboy_service::{http::HttpService, collection::CollectionService};
use postboy_models::{HttpMethod as ModelHttpMethod, Response, Collection};
use std::sync::{Arc, Mutex};
use gpui::Window;
use std::thread;
use crate::theme::get_theme;

/// HTTP methods
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        }
    }

    pub fn color() -> Rgba {
        rgb(0x4ec9b0) // Success color
    }

    /// Convert to model HttpMethod
    pub fn to_model(&self) -> ModelHttpMethod {
        match self {
            HttpMethod::Get => ModelHttpMethod::GET,
            HttpMethod::Post => ModelHttpMethod::POST,
            HttpMethod::Put => ModelHttpMethod::PUT,
            HttpMethod::Delete => ModelHttpMethod::DELETE,
            HttpMethod::Patch => ModelHttpMethod::PATCH,
            HttpMethod::Head => ModelHttpMethod::HEAD,
            HttpMethod::Options => ModelHttpMethod::OPTIONS,
        }
    }
}

/// Header entry for key-value pairs
#[derive(Clone, Debug)]
struct HeaderEntry {
    key: String,
    value: String,
    key_input: Entity<InputState>,
    value_input: Entity<InputState>,
}

/// Query parameter entry for URL parameters
#[derive(Clone, Debug)]
struct QueryParamEntry {
    key: String,
    value: String,
    key_input: Entity<InputState>,
    value_input: Entity<InputState>,
}

/// Authentication types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuthType {
    NoAuth,
    BasicAuth,
    BearerToken,
    ApiKey,
}

impl AuthType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuthType::NoAuth => "No Auth",
            AuthType::BasicAuth => "Basic Auth",
            AuthType::BearerToken => "Bearer Token",
            AuthType::ApiKey => "API Key",
        }
    }

    pub fn all() -> &'static [AuthType] {
        &[AuthType::NoAuth, AuthType::BasicAuth, AuthType::BearerToken, AuthType::ApiKey]
    }

    pub fn next(&self) -> Self {
        match self {
            AuthType::NoAuth => AuthType::BasicAuth,
            AuthType::BasicAuth => AuthType::BearerToken,
            AuthType::BearerToken => AuthType::ApiKey,
            AuthType::ApiKey => AuthType::NoAuth,
        }
    }
}

/// Authentication configuration
#[derive(Clone, Debug)]
pub struct AuthConfig {
    pub auth_type: AuthType,
    // Basic Auth fields
    pub username: String,
    pub password: String,
    username_input: Entity<InputState>,
    password_input: Entity<InputState>,
    // Bearer Token field
    pub token: String,
    token_input: Entity<InputState>,
    // API Key fields
    pub key: String,
    pub value: String,
    pub add_to: String, // "Header" or "Query Params"
    key_input: Entity<InputState>,
    value_input: Entity<InputState>,
    add_to_input: Entity<InputState>,
}

impl AuthConfig {
    fn new(window: &mut Window, cx: &mut Context<MainWindow>) -> Self {
        Self {
            auth_type: AuthType::NoAuth,
            username: String::new(),
            password: String::new(),
            username_input: cx.new(|cx| InputState::new(window, cx)),
            password_input: cx.new(|cx| InputState::new(window, cx)),
            token: String::new(),
            token_input: cx.new(|cx| InputState::new(window, cx)),
            key: String::new(),
            value: String::new(),
            add_to: "Header".to_string(),
            key_input: cx.new(|cx| InputState::new(window, cx)),
            value_input: cx.new(|cx| InputState::new(window, cx)),
            add_to_input: cx.new(|cx| InputState::new(window, cx)),
        }
    }
}

/// History entry for request history
#[derive(Clone, Debug)]
struct HistoryEntry {
    id: usize,
    method: HttpMethod,
    url: String,
    timestamp: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl HistoryEntry {
    fn new(id: usize, method: HttpMethod, url: String, headers: Vec<(String, String)>, body: String) -> Self {
        let now = chrono::Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        Self {
            id,
            method,
            url,
            timestamp,
            headers,
            body,
        }
    }

    fn display_name(&self) -> String {
        format!("{} {}", self.method.as_str(), self.url)
    }
}

/// Response viewer tab
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResponseTab {
    Body,
    Headers,
}

/// Main window view with state
pub struct MainWindow {
    url_input: Entity<InputState>,
    method: HttpMethod,
    headers: Vec<HeaderEntry>,
    query_params: Vec<QueryParamEntry>,
    body_input: Entity<InputState>,
    auth_config: AuthConfig,
    response: Option<Response>,
    response_tab: ResponseTab,
    response_text: String,
    response_status: String,
    response_time: String,
    response_size: String,
    is_loading: bool,
    http_service: Arc<HttpService>,
    pending_response: Arc<Mutex<Option<Result<Response, String>>>>,
    history: Vec<HistoryEntry>,
    next_history_id: usize,
    show_params: bool,
    show_auth: bool,
    collection_service: CollectionService,
    collections: Vec<Collection>,
    collections_loaded: bool,
    show_new_collection_dialog: bool,
    new_collection_name_input: Entity<InputState>,
    new_collection_desc_input: Entity<InputState>,
}

impl MainWindow {
    /// Create a new main window
    pub fn new(window: &mut Window, collection_service: CollectionService, cx: &mut Context<Self>) -> Self {
        let url_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("Enter URL...")
        });

        // Initialize with default headers
        let headers = Self::create_default_headers(window, cx);

        // Initialize body input with placeholder JSON
        let body_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("{\"key\": \"value\"}")
        });

        // Initialize auth configuration
        let auth_config = AuthConfig::new(window, cx);

        // Initialize new collection dialog inputs
        let new_collection_name_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("Collection name")
        });

        let new_collection_desc_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("Description (optional)")
        });

        let mut window_state = Self {
            url_input,
            method: HttpMethod::Get,
            headers,
            query_params: Vec::new(),
            body_input,
            auth_config,
            response: None,
            response_tab: ResponseTab::Body,
            response_text: "Click Send to make a request".to_string(),
            response_status: String::new(),
            response_time: String::new(),
            response_size: String::new(),
            is_loading: false,
            http_service: Arc::new(HttpService::new()),
            pending_response: Arc::new(Mutex::new(None)),
            history: Vec::new(),
            next_history_id: 0,
            show_params: false,
            show_auth: false,
            collection_service,
            collections: Vec::new(),
            collections_loaded: false,
            show_new_collection_dialog: false,
            new_collection_name_input,
            new_collection_desc_input,
        };

        // Load collections asynchronously
        window_state.load_collections(cx);

        window_state
    }

    /// Load collections from the database
    fn load_collections(&mut self, cx: &mut Context<Self>) {
        let collection_service = self.collection_service.clone();

        // Spawn a thread to load collections
        thread::spawn(move || {
            // Create a new tokio runtime for this thread
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    eprintln!("Failed to create runtime: {}", e);
                    return;
                }
            };

            // Execute the async database operation
            let result = rt.block_on(async {
                collection_service.list_collections().await
            });

            // Store the result (we'll poll for it in check_pending_collections)
            // For now, just log the result
            match result {
                Ok(collections) => {
                    eprintln!("Loaded {} collections", collections.len());
                    for collection in &collections {
                        eprintln!("  - {}", collection.name);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load collections: {}", e);
                }
            }
        });

        // For now, mark as loaded immediately
        // In a real implementation, we'd poll for the result
        self.collections_loaded = true;
    }

    /// Create default headers
    fn create_default_headers(window: &mut Window, cx: &mut Context<Self>) -> Vec<HeaderEntry> {
        vec![
            HeaderEntry {
                key: "Content-Type".to_string(),
                value: "application/json".to_string(),
                key_input: cx.new(|cx| InputState::new(window, cx).placeholder("Content-Type")),
                value_input: cx.new(|cx| InputState::new(window, cx).placeholder("application/json")),
            },
            HeaderEntry {
                key: "Accept".to_string(),
                value: "application/json".to_string(),
                key_input: cx.new(|cx| InputState::new(window, cx).placeholder("Accept")),
                value_input: cx.new(|cx| InputState::new(window, cx).placeholder("application/json")),
            },
            HeaderEntry {
                key: "User-Agent".to_string(),
                value: "Postboy/0.1.0".to_string(),
                key_input: cx.new(|cx| InputState::new(window, cx).placeholder("User-Agent")),
                value_input: cx.new(|cx| InputState::new(window, cx).placeholder("Postboy/0.1.0")),
            },
        ]
    }

    /// Add a new header
    fn add_header(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let new_header = HeaderEntry {
            key: String::new(),
            value: String::new(),
            key_input: cx.new(|cx| InputState::new(window, cx).placeholder("Header name")),
            value_input: cx.new(|cx| InputState::new(window, cx).placeholder("Header value")),
        };
        self.headers.push(new_header);
        cx.notify();
    }

    /// Remove a header by index
    fn remove_header(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.headers.len() {
            self.headers.remove(index);
            cx.notify();
        }
    }

    /// Save request to history
    fn save_to_history(&mut self, url: String, headers: Vec<(String, String)>, body: String, cx: &mut Context<Self>) {
        let entry = HistoryEntry::new(
            self.next_history_id,
            self.method,
            url,
            headers,
            body,
        );

        // Add to beginning of history (newest first)
        self.history.insert(0, entry);

        // Limit history to 50 items
        if self.history.len() > 50 {
            self.history.truncate(50);
        }

        self.next_history_id += 1;
        cx.notify();
    }

    /// Load a request from history
    fn load_from_history(&mut self, entry: &HistoryEntry, window: &mut Window, cx: &mut Context<Self>) {
        // Set URL
        self.url_input.update(cx, |input, cx| {
            input.set_value(&entry.url, window, cx);
        });

        // Set method
        self.method = entry.method;

        // Set headers - need to recreate HeaderEntry objects with InputStates
        self.headers.clear();
        for (key, value) in &entry.headers {
            let key_input = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
            let value_input = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));

            key_input.update(cx, |input, cx| {
                input.set_value(key, window, cx);
            });

            value_input.update(cx, |input, cx| {
                input.set_value(value, window, cx);
            });

            self.headers.push(HeaderEntry {
                key: key.clone(),
                value: value.clone(),
                key_input,
                value_input,
            });
        }

        // Set body
        self.body_input.update(cx, |input, cx| {
            input.set_value(&entry.body, window, cx);
        });

        cx.notify();
    }

    /// Clear all history
    fn clear_history(&mut self, cx: &mut Context<Self>) {
        self.history.clear();
        self.next_history_id = 0;
        cx.notify();
    }

    /// Add a new query parameter
    fn add_query_param(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let key_input = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
        let value_input = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));

        self.query_params.push(QueryParamEntry {
            key: String::new(),
            value: String::new(),
            key_input,
            value_input,
        });

        cx.notify();
    }

    /// Remove a query parameter by index
    fn remove_query_param(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.query_params.len() {
            self.query_params.remove(index);
            cx.notify();
        }
    }

    /// Parse URL and extract query parameters
    fn parse_url_params(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let url = self.url_input.read(cx).value().to_string();

        if let Some(query_start) = url.find('?') {
            let base_url = url[..query_start].to_string();
            let query_string = &url[query_start + 1..];

            // Update URL input without query string
            self.url_input.update(cx, |input, cx| {
                input.set_value(&base_url, window, cx);
            });

            // Parse query parameters
            self.query_params.clear();
            for pair in query_string.split('&') {
                if let Some(eq_pos) = pair.find('=') {
                    let key = &pair[..eq_pos];
                    let value = &pair[eq_pos + 1..];

                    // URL decode the key and value
                    let decoded_key = percent_encoding::percent_decode_str(key)
                        .decode_utf8()
                        .unwrap_or_default()
                        .to_string();
                    let decoded_value = percent_encoding::percent_decode_str(value)
                        .decode_utf8()
                        .unwrap_or_default()
                        .to_string();

                    // Create input states for this param
                    let key_input = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
                    let value_input = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));

                    key_input.update(cx, |input, cx| {
                        input.set_value(&decoded_key, window, cx);
                    });

                    value_input.update(cx, |input, cx| {
                        input.set_value(&decoded_value, window, cx);
                    });

                    self.query_params.push(QueryParamEntry {
                        key: decoded_key,
                        value: decoded_value,
                        key_input,
                        value_input,
                    });
                }
            }

            self.show_params = !self.query_params.is_empty();
        }

        cx.notify();
    }

    /// Build URL from base URL and query parameters
    fn build_url_with_params(&self, cx: &mut Context<Self>) -> String {
        let base_url = self.url_input.read(cx).value().to_string();

        if self.query_params.is_empty() {
            return base_url;
        }

        // Build query string
        let params: Vec<String> = self.query_params
            .iter()
            .filter(|p| !p.key_input.read(cx).value().is_empty())
            .map(|p| {
                let key = p.key_input.read(cx).value().to_string();
                let value = p.value_input.read(cx).value().to_string();

                // URL encode the key and value
                let encoded_key = percent_encoding::utf8_percent_encode(&key, percent_encoding::NON_ALPHANUMERIC).to_string();
                let encoded_value = percent_encoding::utf8_percent_encode(&value, percent_encoding::NON_ALPHANUMERIC).to_string();

                format!("{}={}", encoded_key, encoded_value)
            })
            .collect();

        if params.is_empty() {
            base_url
        } else {
            format!("{}?{}", base_url, params.join("&"))
        }
    }

    /// Toggle params visibility
    fn toggle_params(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_params = !self.show_params;

        // If opening params for the first time, parse current URL
        if self.show_params && self.query_params.is_empty() {
            self.parse_url_params(window, cx);
        }

        cx.notify();
    }

    /// Toggle auth visibility
    fn toggle_auth(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.show_auth = !self.show_auth;
        cx.notify();
    }

    /// Cycle auth type
    fn cycle_auth_type(&mut self, cx: &mut Context<Self>) {
        self.auth_config.auth_type = self.auth_config.auth_type.next();
        cx.notify();
    }

    /// Apply authentication headers
    fn apply_auth_headers(&self, headers: &mut Vec<(String, String)>) {
        use base64::prelude::*;

        match self.auth_config.auth_type {
            AuthType::NoAuth => {
                // No authentication
            }
            AuthType::BasicAuth => {
                // Add Basic Auth header
                if !self.auth_config.username.is_empty() || !self.auth_config.password.is_empty() {
                    let credentials = format!("{}:{}", self.auth_config.username, self.auth_config.password);
                    let encoded = BASE64_STANDARD.encode(credentials);
                    headers.push(("Authorization".to_string(), format!("Basic {}", encoded)));
                }
            }
            AuthType::BearerToken => {
                // Add Bearer token header
                if !self.auth_config.token.is_empty() {
                    headers.push(("Authorization".to_string(), format!("Bearer {}", self.auth_config.token)));
                }
            }
            AuthType::ApiKey => {
                // Add API key to header or query params
                if !self.auth_config.key.is_empty() && !self.auth_config.value.is_empty() {
                    if self.auth_config.add_to == "Header" {
                        headers.push((self.auth_config.key.clone(), self.auth_config.value.clone()));
                    }
                    // Query params are handled in build_url_with_params
                }
            }
        }
    }

    /// Update header key
    fn update_header_key(&mut self, index: usize, key: String, cx: &mut Context<Self>) {
        if index < self.headers.len() {
            self.headers[index].key = key;
            cx.notify();
        }
    }

    /// Update header value
    fn update_header_value(&mut self, index: usize, value: String, cx: &mut Context<Self>) {
        if index < self.headers.len() {
            self.headers[index].value = value;
            cx.notify();
        }
    }

    /// Set HTTP method
    fn set_method(&mut self, method: HttpMethod, cx: &mut Context<Self>) {
        self.method = method;
        cx.notify();
    }

    /// Cycle to next HTTP method
    fn cycle_method(&mut self, cx: &mut Context<Self>) {
        self.method = match self.method {
            HttpMethod::Get => HttpMethod::Post,
            HttpMethod::Post => HttpMethod::Put,
            HttpMethod::Put => HttpMethod::Delete,
            HttpMethod::Delete => HttpMethod::Patch,
            HttpMethod::Patch => HttpMethod::Head,
            HttpMethod::Head => HttpMethod::Options,
            HttpMethod::Options => HttpMethod::Get,
        };
        cx.notify();
    }

    /// Handle Send button click
    fn send_request(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        // Build URL with query parameters
        let url = self.build_url_with_params(cx);
        let method = self.method;

        // Validate URL
        if url.trim().is_empty() {
            self.response_text = "Error: URL cannot be empty".to_string();
            self.response_status = "Error".to_string();
            self.response_time.clear();
            cx.notify();
            return;
        }

        self.is_loading = true;
        self.response_text = "Loading...".to_string();
        self.response_status.clear();
        self.response_time.clear();
        cx.notify();

        // Collect headers from UI
        let mut headers: Vec<postboy_models::Header> = self.headers
            .iter()
            .map(|h| postboy_models::Header::new(
                h.key_input.read(cx).value().to_string(),
                h.value_input.read(cx).value().to_string(),
            ))
            .filter(|h| !h.key.is_empty() && !h.value.is_empty())
            .collect();

        // Sync auth input values before applying auth
        self.auth_config.username = self.auth_config.username_input.read(cx).value().to_string();
        self.auth_config.password = self.auth_config.password_input.read(cx).value().to_string();
        self.auth_config.token = self.auth_config.token_input.read(cx).value().to_string();
        self.auth_config.key = self.auth_config.key_input.read(cx).value().to_string();
        self.auth_config.value = self.auth_config.value_input.read(cx).value().to_string();

        // Apply authentication headers
        let mut auth_headers = Vec::new();
        self.apply_auth_headers(&mut auth_headers);
        for (key, value) in auth_headers {
            headers.push(postboy_models::Header::new(key, value));
        }

        // Collect body from UI
        let body_text = self.body_input.read(cx).value().to_string();

        // Determine if we should include body based on method
        let body = if matches!(method, HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch) && !body_text.trim().is_empty() {
            // Try to parse as JSON, if successful use Json body type, otherwise use Raw
            if serde_json::from_str::<serde_json::Value>(&body_text).is_ok() {
                postboy_models::RequestBody::json(body_text)
            } else {
                postboy_models::RequestBody::raw(body_text)
            }
        } else {
            postboy_models::RequestBody::none()
        };

        // Build the request using RequestBuilder
        let mut request_builder = postboy_models::RequestBuilder::new(
            "Manual Request".to_string(),
            method.to_model(),
            url.clone(),
        );

        // Add headers
        for header in headers {
            request_builder = request_builder.header(header.key, header.value);
        }

        // Add body
        request_builder = request_builder.body(body);

        let request = request_builder.build();

        // Spawn a thread to execute the HTTP request
        let http_service = self.http_service.clone();
        let pending_response = self.pending_response.clone();

        // Reset pending response
        *pending_response.lock().unwrap() = None;

        thread::spawn(move || {
            // Create a new tokio runtime for this thread
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    *pending_response.lock().unwrap() = Some(Err(format!("Failed to create runtime: {}", e)));
                    return;
                }
            };

            // Execute the async HTTP request
            let result = rt.block_on(async {
                http_service.send_request(&request).await
            });

            // Store the result
            let response = result.map_err(|e| format!("Request failed: {}", e));
            *pending_response.lock().unwrap() = Some(response);
        });
    }

    /// Set response tab
    fn set_response_tab(&mut self, tab: ResponseTab, cx: &mut Context<Self>) {
        self.response_tab = tab;
        cx.notify();
    }

    /// Check for pending HTTP response and update UI if ready
    fn check_pending_response(&mut self, cx: &mut Context<Self>) {
        let mut pending = self.pending_response.lock().unwrap();
        if let Some(result) = pending.take() {
            self.is_loading = false;

            // Extract data for history BEFORE processing the result
            let url = self.url_input.read(cx).value().to_string();
            let headers: Vec<(String, String)> = self.headers
                .iter()
                .map(|h| (
                    h.key_input.read(cx).value().to_string(),
                    h.value_input.read(cx).value().to_string(),
                ))
                .filter(|(k, v)| !k.is_empty() && !v.is_empty())
                .collect();
            let body = self.body_input.read(cx).value().to_string();

            // Track if request was successful
            let is_success = result.is_ok();

            match result {
                Ok(response) => {
                    // Store the complete response object
                    self.response = Some(response.clone());

                    self.response_status = format!("{} {}", response.status_code, response.status_text);
                    self.response_time = format!("{}ms", response.duration_ms);
                    self.response_size = response.size_str();

                    // Format response body with better error messages for 4xx/5xx
                    self.response_text = match &response.body {
                        postboy_models::ResponseBody::Json(json) => {
                            serde_json::to_string_pretty(json).unwrap_or_else(|_| "Failed to format JSON".to_string())
                        }
                        postboy_models::ResponseBody::Text(text) => text.clone(),
                        postboy_models::ResponseBody::Empty => "<empty response>".to_string(),
                        postboy_models::ResponseBody::Binary(_) => "<binary data>".to_string(),
                    };

                    // Add warning for 4xx and 5xx status codes
                    if response.status_code >= 400 && response.status_code < 600 {
                        let error_type = if response.status_code >= 500 {
                            "Server Error"
                        } else {
                            "Client Error"
                        };
                        self.response_text = format!(
                            "{}\n\n⚠️ {} - The server returned an error status code.",
                            self.response_text, error_type
                        );
                    }
                }
                Err(error) => {
                    // Clear response on error
                    self.response = None;
                    self.response_size.clear();

                    // Categorize and enhance error messages
                    let error_msg = error.to_lowercase();
                    self.response_status = "Error".to_string();
                    self.response_time.clear();

                    if error_msg.contains("timeout") || error_msg.contains("timed out") {
                        self.response_text = format!(
                            "⏱️ Request Timeout\n\n{}\n\nThe request took too long to complete. Please check your network connection or try again later.",
                            error
                        );
                    } else if error_msg.contains("connection") || error_msg.contains("connect") {
                        self.response_text = format!(
                            "🔌 Connection Failed\n\n{}\n\nCould not connect to the server. Please check:\n• The URL is correct\n• You have an internet connection\n• The server is running",
                            error
                        );
                    } else if error_msg.contains("dns") || error_msg.contains("name") {
                        self.response_text = format!(
                            "🌐 DNS Resolution Failed\n\n{}\n\nCould not resolve the hostname. Please check the URL and try again.",
                            error
                        );
                    } else if error_msg.contains("certificate") || error_msg.contains("tls") || error_msg.contains("ssl") {
                        self.response_text = format!(
                            "🔒 SSL/TLS Error\n\n{}\n\nThere was a problem with the secure connection. This could be due to an invalid or expired certificate.",
                            error
                        );
                    } else if error_msg.contains("invalid url") || error_msg.contains("url parse") {
                        self.response_text = format!(
                            "📝 Invalid URL\n\n{}\n\nThe URL format is invalid. Please check and correct it.",
                            error
                        );
                    } else {
                        self.response_text = format!(
                            "❌ Request Failed\n\n{}\n\nAn error occurred while making the request. Please try again.",
                            error
                        );
                    }
                }
            }

            // Drop the lock before calling save_to_history
            drop(pending);

            // Save to history if request was successful
            if is_success {
                self.save_to_history(url, headers, body, cx);
            }

            cx.notify();
        }
    }

    /// Create a new collection
    fn create_collection(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        // Read values from inputs
        let name = self.new_collection_name_input.read(cx).value().to_string();
        let description = self.new_collection_desc_input.read(cx).value().to_string();

        // Validate name
        if name.trim().is_empty() {
            eprintln!("Collection name cannot be empty");
            return;
        }

        let collection_service = self.collection_service.clone();

        // Spawn a thread to create the collection
        thread::spawn(move || {
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    eprintln!("Failed to create runtime: {}", e);
                    return;
                }
            };

            // Execute the async database operation
            let result = rt.block_on(async {
                collection_service.create_collection(name.clone(), if description.trim().is_empty() { None } else { Some(description.clone()) }).await
            });

            match result {
                Ok(_) => {
                    eprintln!("Created collection: {}", name);
                }
                Err(e) => {
                    eprintln!("Failed to create collection: {}", e);
                }
            }
        });

        // Close dialog and clear inputs
        self.show_new_collection_dialog = false;
        self.new_collection_name_input.update(cx, |input, cx| {
            input.set_value("", _window, cx);
        });
        self.new_collection_desc_input.update(cx, |input, cx| {
            input.set_value("", _window, cx);
        });

        // Reload collections
        self.load_collections(cx);

        cx.notify();
    }

    /// Delete a collection
    fn delete_collection(&mut self, collection_id: postboy_models::Id, cx: &mut Context<Self>) {
        let collection_service = self.collection_service.clone();

        // Spawn a thread to delete the collection
        thread::spawn(move || {
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    eprintln!("Failed to create runtime: {}", e);
                    return;
                }
            };

            // Execute the async database operation
            let result = rt.block_on(async {
                collection_service.delete_collection(collection_id).await
            });

            match result {
                Ok(_) => {
                    eprintln!("Deleted collection with id: {}", collection_id);
                }
                Err(e) => {
                    eprintln!("Failed to delete collection: {}", e);
                }
            }
        });

        // Reload collections
        self.load_collections(cx);

        cx.notify();
    }
}

impl Render for MainWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Check for pending HTTP responses
        self.check_pending_response(cx);

        // Get theme colors
        let theme = get_theme();
        let bg = theme.colors.background;
        let surface = theme.colors.surface;
        let border = theme.colors.border;
        let primary = theme.colors.primary;
        let text = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        div()
            .id("main-window")
            .flex()
            .flex_row()
            .size_full()
            .bg(bg)
            .child(
                // Sidebar
                div()
                    .w(px(280.0))
                    .h_full()
                    .bg(surface)
                    .border_r_1()
                    .border_color(border)
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .h(px(48.0))
                            .px_4()
                            .flex()
                            .items_center()
                            .border_b_1()
                            .border_color(border)
                            .child(
                                div()
                                    .text_xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(text)
                                    .child("Postboy")
                            )
                    )
                    .child(
                        div()
                            .flex_1()
                            .p_2()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .mb_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(text_muted)
                                            .child("COLLECTIONS")
                                    )
                                    .child(
                                        Button::new("new-collection-btn")
                                            .small()
                                            .label("+ New")
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.show_new_collection_dialog = true;
                                                cx.notify();
                                            }))
                                    )
                            )
                            .when(!self.collections_loaded, |this_div| {
                                this_div.child(
                                    div()
                                        .text_sm()
                                        .text_color(text_muted)
                                        .child("Loading collections...")
                                )
                            })
                            .when(self.collections_loaded && self.collections.is_empty(), |this_div| {
                                this_div.child(
                                    div()
                                        .text_sm()
                                        .text_color(text_muted)
                                        .child("No collections yet")
                                )
                            })
                            .children(
                                self.collections.iter().map(|collection| {
                                    Self::collection_item_from_model(collection.clone(), &theme, cx)
                                }).collect::<Vec<_>>()
                            )
                    )
                    .child(
                        div()
                            .border_t_1()
                            .border_color(border)
                            .p_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(text_muted)
                                    .mb_2()
                                    .child("ENVIRONMENTS")
                            )
                            .child(Self::environment_item("Production".to_string(), true))
                            .child(Self::environment_item("Development".to_string(), false))
                    )
                    .child(
                        div()
                            .border_t_1()
                            .border_color(border)
                            .p_2()
                            .flex()
                            .flex_col()
                            .max_h(px(300.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .mb_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(text_muted)
                                            .child("HISTORY")
                                    )
                                    .child(
                                        Button::new("clear-history")
                                            .small()
                                            .label("Clear")
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.clear_history(cx);
                                            }))
                                    )
                            )
                            .children(
                                self.history.iter().take(10).map(|entry| {
                                    let entry_clone = entry.clone();
                                    let entry_for_click = entry_clone.clone();
                                    Button::new(("history-item", entry_clone.id))
                                        .small()
                                        .on_click(cx.listener(move |this, _event, window, cx| {
                                            this.load_from_history(&entry_for_click, window, cx);
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_1()
                                                .w_full()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_2()
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(theme.colors.primary)
                                                                .child(entry_clone.method.as_str())
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(text)
                                                                .child(entry_clone.url.clone())
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(text_muted)
                                                        .child(entry_clone.timestamp.clone())
                                                )
                                        )
                                }).collect::<Vec<_>>()
                            )
                    )
            )
            .child(
                // Main content area
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .child(
                        // Request builder section
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .h(px(48.0))
                                    .px_4()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .border_b_1()
                                    .border_color(border)
                                    .bg(surface)
                                    .child(
                                        // Method selector button (cycles through methods on click)
                                        Button::new("method-selector")
                                            .small()
                                            .label(self.method.as_str())
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.cycle_method(cx);
                                            }))
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .child(Input::new(&self.url_input))
                                    )
                                    .child(
                                        // Send button with interaction
                                        Button::new("send-btn")
                                            .primary()
                                            .disabled(self.is_loading)
                                            .label(if self.is_loading { "Sending..." } else { "Send" })
                                            .on_click(cx.listener(Self::send_request))
                                    )
                            )
                            .child(
                                // URL Parameters section
                                div()
                                    .p_4()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(text_muted)
                                            .mb_2()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child("QUERY PARAMETERS")
                                            .child(
                                                div()
                                                    .flex()
                                                    .gap_2()
                                                    .child(
                                                        Button::new("toggle-params")
                                                            .small()
                                                            .label(if self.show_params { "Hide" } else { "Show" })
                                                            .on_click(cx.listener(|this, _event, window, cx| {
                                                                this.toggle_params(window, cx);
                                                            }))
                                                    )
                                                    .child(
                                                        Button::new("add-param")
                                                            .small()
                                                            .label("+ Add")
                                                            .on_click(cx.listener(|this, _event, window, cx| {
                                                                this.add_query_param(window, cx);
                                                            }))
                                                    )
                                            )
                                            .pb_2()
                                            .border_b_1()
                                            .border_color(border)
                                    )
                                    .when(self.show_params, |div| {
                                        div.children(
                                            self.query_params.iter().enumerate().map(|(idx, param)| {
                                                Self::editable_param_row(idx, param, text, border, cx)
                                            }).collect::<Vec<_>>()
                                        )
                                    })
                            )
                            .child(
                                // Authentication section
                                div()
                                    .p_4()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(text_muted)
                                            .mb_2()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child("AUTHENTICATION")
                                            .child(
                                                Button::new("toggle-auth")
                                                    .small()
                                                    .label(if self.show_auth { "Hide" } else { "Show" })
                                                    .on_click(cx.listener(|this, _event, window, cx| {
                                                        this.toggle_auth(window, cx);
                                                    }))
                                            )
                                            .pb_2()
                                            .border_b_1()
                                            .border_color(border)
                                    )
                                    .when(self.show_auth, |parent_div| {
                                        // Build auth content based on type
                                        let mut children = vec![
                                            // Auth type selector (always shown)
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_2()
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .text_color(text_muted)
                                                                .child("Type:")
                                                        )
                                                        .child(
                                                            Button::new("auth-type-selector")
                                                                .small()
                                                                .label(self.auth_config.auth_type.as_str())
                                                                .on_click(cx.listener(|this, _event, _window, cx| {
                                                                    this.cycle_auth_type(cx);
                                                                }))
                                                        )
                                                        .into_element()
                                                ];

                                        // Add type-specific content
                                        if self.auth_config.auth_type == AuthType::NoAuth {
                                            children.push(
                                                div()
                                                    .p_3()
                                                    .bg(theme.colors.background_sunken)
                                                    .rounded(px(4.0))
                                                    .text_sm()
                                                    .text_color(text_muted)
                                                    .child("No authentication will be used with this request.")
                                                    .into_element()
                                            );
                                        } else if self.auth_config.auth_type == AuthType::BasicAuth {
                                            children.push(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap_1()
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_weight(FontWeight::SEMIBOLD)
                                                                    .text_color(text_muted)
                                                                    .child("Username")
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex_1()
                                                                    .child(
                                                                        Input::new(&self.auth_config.username_input)
                                                                    )
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap_1()
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_weight(FontWeight::SEMIBOLD)
                                                                    .text_color(text_muted)
                                                                    .child("Password")
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex_1()
                                                                    .child(
                                                                        Input::new(&self.auth_config.password_input)
                                                                    )
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(text_muted)
                                                            .child("Basic Auth will add Authorization header with Base64 encoded credentials.")
                                                    )
                                                    .into_element()
                                            );
                                        } else if self.auth_config.auth_type == AuthType::BearerToken {
                                            children.push(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap_1()
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_weight(FontWeight::SEMIBOLD)
                                                                    .text_color(text_muted)
                                                                    .child("Token")
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex_1()
                                                                    .child(
                                                                        Input::new(&self.auth_config.token_input)
                                                                    )
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(text_muted)
                                                            .child("Bearer token will be added to the Authorization header.")
                                                    )
                                                    .into_element()
                                            );
                                        } else if self.auth_config.auth_type == AuthType::ApiKey {
                                            children.push(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap_1()
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_weight(FontWeight::SEMIBOLD)
                                                                    .text_color(text_muted)
                                                                    .child("Key")
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex_1()
                                                                    .child(
                                                                        Input::new(&self.auth_config.key_input)
                                                                    )
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap_1()
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_weight(FontWeight::SEMIBOLD)
                                                                    .text_color(text_muted)
                                                                    .child("Value")
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex_1()
                                                                    .child(
                                                                        Input::new(&self.auth_config.value_input)
                                                                    )
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap_1()
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_weight(FontWeight::SEMIBOLD)
                                                                    .text_color(text_muted)
                                                                    .child("Add to")
                                                            )
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .gap_2()
                                                                    .child(
                                                                        Button::new("add-to-header-btn")
                                                                            .small()
                                                                            .label("Header")
                                                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                                                this.auth_config.add_to = "Header".to_string();
                                                                                cx.notify();
                                                                            }))
                                                                    )
                                                                    .child(
                                                                        Button::new("add-to-query-btn")
                                                                            .small()
                                                                            .label("Query Params")
                                                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                                                this.auth_config.add_to = "Query Params".to_string();
                                                                                cx.notify();
                                                                            }))
                                                                    )
                                                                    .child(
                                                                        div()
                                                                            .text_xs()
                                                                            .text_color(text_muted)
                                                                            .child(format!("(currently: {})", self.auth_config.add_to))
                                                                    )
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(text_muted)
                                                            .child("API key will be sent in the request header.")
                                                    )
                                                    .into_element()
                                            );
                                        }

                                        parent_div.children(children)
                                    })
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .p_4()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(text_muted)
                                            .mb_2()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child("HEADERS")
                                            .child(
                                                Button::new("add-header")
                                                    .small()
                                                    .label("+ Add")
                                                    .on_click(cx.listener(|this, _event, window, cx| {
                                                        this.add_header(window, cx);
                                                    }))
                                            )
                                            .pb_2()
                                            .border_b_1()
                                            .border_color(border)
                                    )
                                    .children(
                                        self.headers.iter().enumerate().map(|(idx, header)| {
                                            Self::editable_header_row(idx, header, text, border, cx)
                                        }).collect::<Vec<_>>()
                                    )
                            )
                            .child(
                                // Body editor section
                                div()
                                    .flex_1()
                                    .p_4()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(text_muted)
                                            .mb_2()
                                            .child("BODY")
                                            .pb_2()
                                            .border_b_1()
                                            .border_color(border)
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .min_h(px(120.0))
                                            .child(Input::new(&self.body_input))
                                    )
                            )
                    )
                    .child(
                        div()
                            .h(px(1.0))
                            .bg(border)
                    )
                    .child(
                        // Response viewer section
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .min_h(px(40.0))
                                    .px_4()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .border_b_1()
                                    .border_color(border)
                                    .bg(surface)
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(text_muted)
                                            .child("Response")
                                    )
                                    .child(
                                        if !self.response_status.is_empty() {
                                            let status_color = if self.response_status == "Error" {
                                                theme.colors.error  // Request errors
                                            } else if self.response_status.starts_with("2") {
                                                theme.colors.success  // Success
                                            } else if self.response_status.starts_with("4") {
                                                theme.colors.warning  // Client errors
                                            } else if self.response_status.starts_with("5") {
                                                theme.colors.error  // Server errors
                                            } else if self.response_status.starts_with("3") {
                                                theme.colors.info  // Redirects
                                            } else {
                                                theme.colors.text_muted  // Other
                                            };

                                            div()
                                                .px_2()
                                                .py(px(2.0))
                                                .bg(status_color)
                                                .rounded(px(3.0))
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0xffffff))
                                                .child(self.response_status.clone())
                                        } else {
                                            div()
                                                .px_2()
                                                .py(px(2.0))
                                                .bg(border)
                                                .rounded(px(3.0))
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0xffffff))
                                                .child("200 OK")
                                        }
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(text_muted)
                                            .child(format!("{} | {}", self.response_time, self.response_size))
                                    )
                            )
                            .child(
                                // Tab selector
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .px_4()
                                    .py_2()
                                    .border_b_1()
                                    .border_color(border)
                                    .bg(surface)
                                    .child(
                                        Button::new("tab-body")
                                            .small()
                                            .label("Body")
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.set_response_tab(ResponseTab::Body, cx);
                                            }))
                                    )
                                    .child(
                                        Button::new("tab-headers")
                                            .small()
                                            .label("Headers")
                                            .on_click(cx.listener(|this, _event, _window, cx| {
                                                this.set_response_tab(ResponseTab::Headers, cx);
                                            }))
                                    )
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .p_4()
                                    .bg(bg)
                                    .child(
                                        if self.is_loading {
                                            // Loading indicator
                                            div()
                                                .flex()
                                                .flex_col()
                                                .items_center()
                                                .justify_center()
                                                .gap_4()
                                                .min_h(px(200.0))
                                                .child(
                                                    div()
                                                        .text_2xl()
                                                        .child("⏳")
                                                )
                                                .child(
                                                    div()
                                                        .text_lg()
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(theme.colors.text_muted)
                                                        .child("Sending request...")
                                                )
                                                .child(
                                                    div()
                                                        .text_sm()
                                                        .text_color(theme.colors.text_disabled)
                                                        .child("Please wait")
                                                )
                                        } else if self.response_tab == ResponseTab::Headers {
                                            // Headers tab
                                            if let Some(response) = &self.response {
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_2()
                                                    .children(
                                                        response.headers.iter().map(|header| {
                                                            div()
                                                                .flex()
                                                                .items_start()
                                                                .gap_2()
                                                                .child(
                                                                    div()
                                                                        .text_sm()
                                                                        .font_weight(FontWeight::SEMIBOLD)
                                                                        .text_color(theme.colors.primary)
                                                                        .child(header.name.clone())
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_sm()
                                                                        .text_color(text)
                                                                        .child(header.value.clone())
                                                                )
                                                        }).collect::<Vec<_>>()
                                                    )
                                            } else {
                                                div()
                                                    .text_sm()
                                                    .text_color(text_muted)
                                                    .child("No response headers available")
                                            }
                                        } else {
                                            // Body tab (default)
                                            div()
                                                .text_xs()
                                                .text_color(text)
                                                .child(self.response_text.clone())
                                        }
                                    )
                            )
                    )
            )
            .when(self.show_new_collection_dialog, |div| {
                div.child(Self::render_new_collection_dialog(cx, &self.new_collection_name_input, &self.new_collection_desc_input, &theme))
            })
    }
}

impl MainWindow {
    fn render_new_collection_dialog(
        cx: &mut Context<Self>,
        name_input: &Entity<InputState>,
        desc_input: &Entity<InputState>,
        theme: &crate::theme::Theme
    ) -> Div {
        let surface = theme.colors.surface;
        let border = theme.colors.border;
        let text = theme.colors.text;
        let text_muted = theme.colors.text_muted;

        div()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(0x000000aa))
            .child(
                div()
                    .w(px(400.0))
                    .bg(surface)
                    .border_1()
                    .border_color(border)
                    .rounded(px(8.0))
                    .shadow_lg()
                    .child(
                        div()
                            .p_4()
                            .border_b_1()
                            .border_color(border)
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(text)
                                    .child("Create New Collection")
                            )
                    )
                    .child(
                        div()
                            .p_4()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(text_muted)
                                            .child("Name")
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .child(Input::new(name_input))
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(text_muted)
                                            .child("Description")
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .child(Input::new(desc_input))
                                    )
                            )
                    )
                    .child(
                        div()
                            .p_4()
                            .border_t_1()
                            .border_color(border)
                            .flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                Button::new("cancel-collection-btn")
                                    .small()
                                    .label("Cancel")
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.show_new_collection_dialog = false;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                Button::new("create-collection-btn")
                                    .small()
                                    .primary()
                                    .label("Create")
                                    .on_click(cx.listener(Self::create_collection))
                            )
                    )
            )
    }

    fn collection_item(name: String) -> Div {
        let theme = get_theme();
        div()
            .px_3()
            .py_2()
            .rounded(px(4.0))
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.surface_hover))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().text_lg().child("📁"))
                    .child(div().text_sm().text_color(theme.colors.text).child(name))
            )
    }

    fn collection_item_from_model(collection: Collection, theme: &crate::theme::Theme, cx: &mut Context<Self>) -> Div {
        let collection_id = collection.id;
        // Use a hash of the UUID to create a usize for the button ID
        let collection_hash = collection_id.as_u128() as usize;
        div()
            .px_3()
            .py_2()
            .rounded(px(4.0))
            .hover(|s| s.bg(theme.colors.surface_hover))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_lg().child("📁"))
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.colors.text)
                                    .child(collection.name.clone())
                            )
                    )
                    .child(
                        Button::new(("delete-collection", collection_hash))
                            .small()
                            .label("×")
                            .on_click(cx.listener(move |this, _event, _window, cx| {
                                this.delete_collection(collection_id, cx);
                            }))
                    )
            )
    }

    fn environment_item(name: String, active: bool) -> Div {
        let theme = get_theme();
        div()
            .px_3()
            .py_2()
            .rounded(px(4.0))
            .cursor_pointer()
            .hover(|s| s.bg(theme.colors.surface_hover))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_color(if active { theme.colors.primary } else { theme.colors.text_disabled })
                            .child(if active { "●" } else { "○" })
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(if active { theme.colors.primary } else { theme.colors.text_muted })
                            .child(name)
                    )
            )
    }

    fn header_row(key: String, value: String, text: Rgba, border: Rgba) -> Div {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .w(px(16.0))
                    .h(px(16.0))
                    .border_1()
                    .border_color(rgb(0x007acc))
                    .rounded(px(2.0))
                    .bg(rgb(0x007acc))
            )
            .child(
                div()
                    .flex_1()
                    .h(px(32.0))
                    .px_3()
                    .bg(rgb(0x1e1e1e))
                    .border_1()
                    .border_color(border)
                    .rounded(px(2.0))
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(text)
                            .child(key)
                    )
            )
            .child(
                div()
                    .flex_1()
                    .h(px(32.0))
                    .px_3()
                    .bg(rgb(0x1e1e1e))
                    .border_1()
                    .border_color(border)
                    .rounded(px(2.0))
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(text)
                            .child(value)
                    )
            )
    }

    fn editable_header_row(idx: usize, header: &HeaderEntry, text: Rgba, border: Rgba, cx: &mut Context<Self>) -> Div {
        let header_idx = idx;
        let key_input = header.key_input.clone();
        let value_input = header.value_input.clone();

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .w(px(16.0))
                    .h(px(16.0))
                    .border_1()
                    .border_color(rgb(0x007acc))
                    .rounded(px(2.0))
                    .bg(rgb(0x007acc))
            )
            .child(
                div()
                    .flex_1()
                    .child(Input::new(&key_input))
            )
            .child(
                div()
                    .flex_1()
                    .child(Input::new(&value_input))
            )
            .child(
                Button::new(("delete-header", header_idx))
                    .small()
                    .label("×")
                    .on_click(cx.listener(move |this, _event, _window, cx| {
                        this.remove_header(header_idx, cx);
                    }))
            )
    }

    /// Create an editable query parameter row
    fn editable_param_row(idx: usize, param: &QueryParamEntry, text: Rgba, border: Rgba, cx: &mut Context<Self>) -> Div {
        let param_idx = idx;
        let key_input = param.key_input.clone();
        let value_input = param.value_input.clone();

        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .w(px(16.0))
                    .h(px(16.0))
                    .border_1()
                    .border_color(rgb(0x4ec9b0))
                    .rounded(px(2.0))
                    .bg(rgb(0x4ec9b0))
            )
            .child(
                div()
                    .flex_1()
                    .child(Input::new(&key_input))
            )
            .child(
                div()
                    .flex_1()
                    .child(Input::new(&value_input))
            )
            .child(
                Button::new(("delete-param", param_idx))
                    .small()
                    .label("×")
                    .on_click(cx.listener(move |this, _event, _window, cx| {
                        this.remove_query_param(param_idx, cx);
                    }))
            )
    }
}
