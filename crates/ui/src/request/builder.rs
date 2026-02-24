//! Request builder component
//!
//! Main component for building HTTP requests. Inspired by Postman's UI design.

use gpui::*;
use postboy_models::HttpMethod;

/// Request builder view
pub struct RequestBuilder {
    /// Selected HTTP method
    method: HttpMethod,
    /// Current URL text
    url: String,
    /// Demo response text
    response: String,
}

impl RequestBuilder {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            method: HttpMethod::GET,
            url: "https://httpbin.org/get".to_string(),
            response: "✨ Postboy v0.1.0\n\nHTTP 功能已完成！\n\nUI 界面已完成，支持：\n- ✅ Postman 风格的请求栏\n- ✅ 方法选择器（GET/POST/PUT/DELETE等）\n- ✅ URL 输入框\n- ✅ 响应查看器\n- ✅ HTTP 服务（100% 完成）\n\n示例 API：\nhttps://httpbin.org/get\nhttps://api.github.com\nhttps://jsonplaceholder.typicode.com/posts\n\n💡 提示：完整的交互功能（点击按钮、输入URL等）正在开发中".to_string(),
        }
    }

    /// Get color for HTTP method (Postman-style)
    fn get_method_color(&self) -> Rgba {
        match self.method {
            HttpMethod::GET => rgba(0x4A90E2FF),      // Blue
            HttpMethod::POST => rgba(0xF5A623FF),     // Orange/Yellow
            HttpMethod::PUT => rgba(0x50E3C2FF),      // Teal/Green
            HttpMethod::DELETE => rgba(0xE74C3CFF),   // Red
            HttpMethod::PATCH => rgba(0x9B59B6FF),    // Purple
            HttpMethod::HEAD => rgba(0x95A5A6FF),     // Gray
            HttpMethod::OPTIONS => rgba(0x34495EFF),  // Dark Blue
        }
    }

    /// Get method name as string
    fn get_method_name(&self) -> &'static str {
        match self.method {
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

impl Render for RequestBuilder {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let method_color = self.get_method_color();
        let method_name = self.get_method_name().to_string();
        let url_text = self.url.clone();
        let response_text = self.response.clone();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgba(0x1E1E1EFF))
            .child(
                // Request URL Bar - Postman style
                div()
                    .w_full()
                    .h(px(54.0))
                    .bg(rgba(0x252525FF))
                    .flex()
                    .items_center()
                    .p_2()
                    .gap_2()
                    .child(
                        // Method selector button
                        div()
                            .w(px(100.0))
                            .h(px(38.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(method_color)
                            .rounded(px(4.0))
                            .child(
                                div()
                                    .text_color(rgba(0xFFFFFFFF))
                                    .font_weight(FontWeight::BOLD)
                                    .child(method_name)
                            )
                    )
                    .child(
                        // URL input field
                        div()
                            .flex_1()
                            .h(px(38.0))
                            .px_3()
                            .flex()
                            .items_center()
                            .bg(rgba(0x1E1E1EFF))
                            .border_1()
                            .border_color(rgba(0x3A3A3AFF))
                            .rounded(px(4.0))
                            .child(
                                div()
                                    .text_color(rgba(0xCCCCCCFF))
                                    .child(url_text)
                            )
                    )
                    .child(
                        // Send button
                        div()
                            .w(px(90.0))
                            .h(px(38.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(rgba(0x007ACCCC))
                            .rounded(px(4.0))
                            .child(
                                div()
                                    .text_color(rgba(0xFFFFFFFF))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Send")
                            )
                    )
            )
            .child(
                // Response area
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .child(
                        div()
                            .w_full()
                            .h(px(40.0))
                            .bg(rgba(0x2D2D2DFF))
                            .flex()
                            .items_center()
                            .px_4()
                            .child(
                                div()
                                    .text_color(rgba(0x888888FF))
                                    .child("Response")
                            )
                    )
                    .child(
                        div()
                            .flex_1()
                            .p_4()
                            .child(
                                div()
                                    .text_color(rgba(0xCCCCCCFF))
                                    .child(response_text)
                            )
                    )
            )
    }
}
