//! Response formatting utilities
//!
//! Helper functions for formatting HTTP responses for display.

use serde_json::Value;

/// Format response status with emoji indicator
pub fn format_status(status_code: u16) -> String {
    let icon = if status_code >= 200 && status_code < 300 {
        "✅"
    } else if status_code >= 300 && status_code < 400 {
        "↪️"
    } else if status_code >= 400 && status_code < 500 {
        "⚠️"
    } else if status_code >= 500 {
        "❌"
    } else {
        "ℹ️"
    };

    format!("{} {}", icon, status_code)
}

/// Format response body for display
pub fn format_body(body: &str, content_type: Option<&str>) -> String {
    // Check if content is JSON
    let is_json = content_type
        .map(|ct| ct.contains("json"))
        .unwrap_or(false);

    if is_json {
        // Try to parse and format JSON
        if let Ok(value) = serde_json::from_str::<Value>(body) {
            if let Ok(formatted) = serde_json::to_string_pretty(&value) {
                return formatted;
            }
        }
    }

    // Return body as-is if not JSON or parsing failed
    body.to_string()
}

/// Truncate body if too long
pub fn truncate_body(body: &str, max_chars: usize) -> String {
    if body.len() <= max_chars {
        return body.to_string();
    }

    format!("{}\n\n... (truncated, {} bytes total)",
        &body[..max_chars],
        body.len()
    )
}

/// Format headers for display
pub fn format_headers(headers: &[(String, String)]) -> String {
    headers
        .iter()
        .map(|(k, v)| format!("  {}: {}", k, v))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Format duration for display
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

/// Format bytes for display
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
    fn test_format_status() {
        assert_eq!(format_status(200), "✅ 200");
        assert_eq!(format_status(301), "↪️ 301");
        assert_eq!(format_status(404), "⚠️ 404");
        assert_eq!(format_status(500), "❌ 500");
        assert_eq!(format_status(100), "ℹ️ 100");
    }

    #[test]
    fn test_format_body_json() {
        let json = r#"{"name":"test","value":123}"#;
        let formatted = format_body(json, Some("application/json"));
        assert!(formatted.contains("\n"));
        assert!(formatted.contains("name"));
    }

    #[test]
    fn test_format_body_text() {
        let text = "plain text response";
        let formatted = format_body(text, Some("text/plain"));
        assert_eq!(formatted, text);
    }

    #[test]
    fn test_truncate_body() {
        let long_body = "a".repeat(1000);
        let truncated = truncate_body(&long_body, 100);
        assert!(truncated.len() < long_body.len());
        assert!(truncated.contains("truncated"));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(500), "500ms");
        assert_eq!(format_duration(1500), "1.5s");
        assert_eq!(format_duration(65000), "1m 5s");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(100), "100B");
        assert_eq!(format_bytes(2048), "2.00KB");
        assert_eq!(format_bytes(3_145_728), "3.00MB");
    }
}
