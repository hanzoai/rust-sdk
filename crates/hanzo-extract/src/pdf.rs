//! PDF document content extraction

use crate::{config::ExtractorConfig, error::Result, ExtractError, ExtractResult, Extractor};
use lopdf::Document;
use std::path::Path;

/// PDF document content extractor
pub struct PdfExtractor {
    config: ExtractorConfig,
}

impl PdfExtractor {
    /// Create a new PDF extractor with the given configuration
    pub fn new(config: ExtractorConfig) -> Self {
        Self { config }
    }
    
    /// Create a new PDF extractor with default configuration
    pub fn default() -> Self {
        Self::new(ExtractorConfig::default())
    }
    
    /// Extract text from a PDF file path
    pub fn extract_from_file(&self, path: &Path) -> Result<ExtractResult> {
        let doc = Document::load(path)?;
        self.extract_from_document(&doc, path.to_string_lossy().to_string())
    }
    
    /// Extract text from PDF bytes
    pub fn extract_from_bytes(&self, bytes: &[u8], source: String) -> Result<ExtractResult> {
        let doc = Document::load_mem(bytes)?;
        self.extract_from_document(&doc, source)
    }
    
    /// Extract text from a lopdf Document
    fn extract_from_document(&self, doc: &Document, source: String) -> Result<ExtractResult> {
        let mut text_parts: Vec<String> = Vec::new();
        let pages = doc.get_pages();
        
        for (page_num, _) in pages.iter() {
            if let Ok(page_text) = doc.extract_text(&[*page_num]) {
                let cleaned = self.clean_text(&page_text);
                if !cleaned.is_empty() {
                    text_parts.push(cleaned);
                }
            }
        }
        
        let text = text_parts.join("\n\n");
        
        if text.len() > self.config.max_length {
            return Err(ExtractError::ContentTooLarge {
                size: text.len(),
                max: self.config.max_length,
            });
        }
        
        let mut result = ExtractResult::new(text, source)
            .with_content_type("application/pdf")
            .with_metadata("page_count", pages.len().to_string());
        
        // Extract title from PDF metadata if available
        if let Ok(catalog) = doc.catalog() {
            if let Ok(info_ref) = catalog.get(b"Info") {
                if let Ok(info) = doc.get_object(info_ref.as_reference().unwrap_or_default()) {
                    if let Ok(info_dict) = info.as_dict() {
                        if let Ok(title) = info_dict.get(b"Title") {
                            if let Ok(title_bytes) = title.as_str() {
                                if let Ok(title_str) = std::str::from_utf8(title_bytes) {
                                    result = result.with_title(title_str);
                                }
                            }
                        }
                        if let Ok(author) = info_dict.get(b"Author") {
                            if let Ok(author_bytes) = author.as_str() {
                                if let Ok(author_str) = std::str::from_utf8(author_bytes) {
                                    result = result.with_metadata("author", author_str);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    /// Clean extracted text
    fn clean_text(&self, text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut prev_was_whitespace = false;
        
        for c in text.chars() {
            if c.is_whitespace() {
                if !prev_was_whitespace {
                    result.push(if c == '\n' { '\n' } else { ' ' });
                    prev_was_whitespace = true;
                }
            } else {
                result.push(c);
                prev_was_whitespace = false;
            }
        }
        
        result.trim().to_string()
    }
}

#[async_trait::async_trait]
impl Extractor for PdfExtractor {
    /// Extract text from a PDF source (file path or URL)
    async fn extract(&self, source: &str) -> Result<ExtractResult> {
        // Check if source is a URL
        if source.starts_with("http://") || source.starts_with("https://") {
            #[cfg(feature = "web")]
            {
                // Fetch PDF from URL
                let client = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(self.config.timeout_secs))
                    .build()
                    .map_err(|e| ExtractError::Network(e.to_string()))?;
                
                let response = client.get(source).send().await
                    .map_err(|e| ExtractError::Network(e.to_string()))?;
                
                if !response.status().is_success() {
                    return Err(ExtractError::Http {
                        status: response.status().as_u16(),
                        message: response.status().to_string(),
                    });
                }
                
                let bytes = response.bytes().await
                    .map_err(|e| ExtractError::Network(e.to_string()))?;
                
                return self.extract_from_bytes(&bytes, source.to_string());
            }
            
            #[cfg(not(feature = "web"))]
            {
                return Err(ExtractError::Other(
                    "URL extraction requires 'web' feature".to_string()
                ));
            }
        }
        
        // Treat as file path
        let path = Path::new(source);
        if !path.exists() {
            return Err(ExtractError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", source),
            )));
        }
        
        self.extract_from_file(path)
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
        let extractor = PdfExtractor::default();
        let input = "  Hello   World  \n\n  Test  ";
        let result = extractor.clean_text(input);
        // All consecutive whitespace collapsed to single space
        assert_eq!(result, "Hello World Test");
    }
}
