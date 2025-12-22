//! Web page content extraction

use crate::{config::ExtractorConfig, error::Result, ExtractError, ExtractResult, Extractor};
use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Duration;

/// Web page content extractor
pub struct WebExtractor {
    config: ExtractorConfig,
    client: Client,
}

impl Default for WebExtractor {
    fn default() -> Self {
        Self::new(ExtractorConfig::default())
    }
}

impl WebExtractor {
    /// Create a new web extractor with the given configuration
    pub fn new(config: ExtractorConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .redirect(if config.follow_redirects {
                reqwest::redirect::Policy::limited(config.max_redirects)
            } else {
                reqwest::redirect::Policy::none()
            })
            .user_agent(&config.user_agent)
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }

    /// Extract clean text from HTML content
    fn extract_text_from_html(&self, html: &str) -> (String, Option<String>) {
        let document = Html::parse_document(html);

        // Extract title
        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string());

        // Remove script and style elements
        let mut text_parts = Vec::new();

        // Try to get main content areas first
        let content_selectors = [
            "article",
            "main",
            "[role='main']",
            ".content",
            ".post-content",
            ".article-content",
            "#content",
            "#main",
        ];

        let mut found_main_content = false;
        for selector_str in content_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    let text = self.extract_element_text(&element);
                    if !text.is_empty() {
                        text_parts.push(text);
                        found_main_content = true;
                    }
                }
                if found_main_content {
                    break;
                }
            }
        }

        // Fall back to body if no main content found
        if !found_main_content {
            if let Ok(body_selector) = Selector::parse("body") {
                for element in document.select(&body_selector) {
                    text_parts.push(self.extract_element_text(&element));
                }
            }
        }

        let text = text_parts.join("\n\n");
        let clean_text = self.clean_text(&text);

        (clean_text, title)
    }

    /// Extract text from an HTML element, skipping script/style
    #[allow(clippy::only_used_in_recursion)]
    fn extract_element_text(&self, element: &scraper::ElementRef) -> String {
        let skip_tags = [
            "script", "style", "noscript", "nav", "header", "footer", "aside",
        ];

        let mut text = String::new();
        for child in element.children() {
            if let Some(element) = child.value().as_element() {
                let tag = element.name();
                if skip_tags.contains(&tag) {
                    continue;
                }
                if let Some(child_element) = scraper::ElementRef::wrap(child) {
                    text.push_str(&self.extract_element_text(&child_element));
                    if matches!(
                        tag,
                        "p" | "div" | "br" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "li"
                    ) {
                        text.push('\n');
                    }
                }
            } else if let Some(text_node) = child.value().as_text() {
                text.push_str(text_node);
            }
        }
        text
    }

    /// Clean extracted text
    fn clean_text(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut prev_was_whitespace = false;
        let mut prev_was_newline = false;
        let mut newline_count = 0;

        for c in text.chars() {
            if c == '\n' {
                newline_count += 1;
                if newline_count <= 2 && !prev_was_newline {
                    result.push('\n');
                    prev_was_newline = true;
                }
                prev_was_whitespace = true;
            } else if c.is_whitespace() {
                if !prev_was_whitespace {
                    result.push(' ');
                    prev_was_whitespace = true;
                }
                newline_count = 0;
            } else {
                result.push(c);
                prev_was_whitespace = false;
                prev_was_newline = false;
                newline_count = 0;
            }
        }

        result.trim().to_string()
    }
}

#[async_trait::async_trait]
impl Extractor for WebExtractor {
    async fn extract(&self, source: &str) -> Result<ExtractResult> {
        // Validate URL
        let url =
            url::Url::parse(source).map_err(|_| ExtractError::InvalidUrl(source.to_string()))?;

        // Fetch the page
        let response = self.client.get(url.as_str()).send().await?;

        let status = response.status();
        if !status.is_success() {
            return Err(ExtractError::Http {
                status: status.as_u16(),
                message: status.to_string(),
            });
        }

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let body = response.text().await?;
        let original_length = body.len();

        // Check size limit
        if original_length > self.config.max_length {
            return Err(ExtractError::ContentTooLarge {
                size: original_length,
                max: self.config.max_length,
            });
        }

        // Extract text
        let (text, title) = if self.config.clean_text {
            self.extract_text_from_html(&body)
        } else {
            (body, None)
        };

        let mut result =
            ExtractResult::new(text, source.to_string()).with_original_length(original_length);

        if let Some(ct) = content_type {
            result = result.with_content_type(ct);
        }

        if let Some(t) = title {
            result = result.with_title(t);
        }

        Ok(result)
    }

    #[cfg(feature = "sanitize")]
    async fn extract_sanitized(&self, source: &str) -> Result<ExtractResult> {
        let result = self.extract(source).await?;
        crate::sanitize::sanitize_result(result, &self.config).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text() {
        let extractor = WebExtractor::default();
        let input = "  Hello   World  \n\n\n\n  Test  ";
        let result = extractor.clean_text(input);
        // Consecutive whitespace collapsed, preserving single newline
        assert_eq!(result, "Hello World \nTest");
    }

    #[test]
    fn test_extract_text_from_html() {
        let extractor = WebExtractor::default();
        let html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Test Page</title></head>
        <body>
            <script>alert('ignore me')</script>
            <h1>Hello World</h1>
            <p>This is a test paragraph.</p>
        </body>
        </html>
        "#;

        let (text, title) = extractor.extract_text_from_html(html);
        assert_eq!(title, Some("Test Page".to_string()));
        assert!(text.contains("Hello World"));
        assert!(text.contains("This is a test paragraph"));
        assert!(!text.contains("alert"));
    }
}
