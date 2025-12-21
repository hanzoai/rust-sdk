//! Content sanitization using hanzo-guard

use crate::{
    config::ExtractorConfig,
    error::{ExtractError, Result},
    result::{ExtractResult, SanitizationInfo},
};
use hanzo_guard::{
    config::GuardConfig,
    Guard,
    types::SanitizeResult as GuardResult,
};

/// Sanitize extraction result using hanzo-guard
pub async fn sanitize_result(mut result: ExtractResult, config: &ExtractorConfig) -> Result<ExtractResult> {
    let mut guard_config = GuardConfig::default();
    guard_config.pii.enabled = config.redact_pii;
    guard_config.injection.enabled = config.detect_injection;
    guard_config.rate_limit.enabled = false; // Don't rate limit during extraction
    guard_config.content_filter.enabled = false; // Skip external API calls
    
    let guard = Guard::new(guard_config);
    let sanitize_result = guard.sanitize_input(&result.text).await
        .map_err(|e| ExtractError::Other(e.to_string()))?;
    
    match sanitize_result {
        GuardResult::Clean(text) => {
            result.text = text;
            result.text_length = result.text.len();
            result.sanitized = true;
            result.sanitization = Some(SanitizationInfo::default());
            Ok(result)
        }
        GuardResult::Redacted { text, redactions } => {
            let pii_types: Vec<String> = redactions
                .iter()
                .map(|r| format!("{:?}", r.redaction_type))
                .collect();
            
            let sanitization = SanitizationInfo {
                pii_redacted: redactions.len(),
                pii_types,
                injection_detected: false,
                injection_confidence: 0.0,
                blocked: false,
                block_reason: None,
            };
            
            result.text = text;
            result.text_length = result.text.len();
            result.sanitized = true;
            result.sanitization = Some(sanitization);
            Ok(result)
        }
        GuardResult::Blocked { reason, .. } => {
            Err(ExtractError::Blocked(reason))
        }
    }
}

/// Sanitize text directly and return the cleaned version
pub async fn sanitize_text(text: &str, config: &ExtractorConfig) -> Result<(String, SanitizationInfo)> {
    let mut guard_config = GuardConfig::default();
    guard_config.pii.enabled = config.redact_pii;
    guard_config.injection.enabled = config.detect_injection;
    guard_config.rate_limit.enabled = false;
    guard_config.content_filter.enabled = false;
    
    let guard = Guard::new(guard_config);
    let sanitize_result = guard.sanitize_input(text).await
        .map_err(|e| ExtractError::Other(e.to_string()))?;
    
    match sanitize_result {
        GuardResult::Clean(text) => {
            Ok((text, SanitizationInfo::default()))
        }
        GuardResult::Redacted { text, redactions } => {
            let pii_types: Vec<String> = redactions
                .iter()
                .map(|r| format!("{:?}", r.redaction_type))
                .collect();
            
            let sanitization = SanitizationInfo {
                pii_redacted: redactions.len(),
                pii_types,
                injection_detected: false,
                injection_confidence: 0.0,
                blocked: false,
                block_reason: None,
            };
            
            Ok((text, sanitization))
        }
        GuardResult::Blocked { reason, .. } => {
            Err(ExtractError::Blocked(reason))
        }
    }
}

/// Check if text contains any safety issues without modifying it
pub async fn check_safety(text: &str, config: &ExtractorConfig) -> (bool, Option<String>) {
    let mut guard_config = GuardConfig::default();
    guard_config.pii.enabled = config.redact_pii;
    guard_config.injection.enabled = config.detect_injection;
    guard_config.rate_limit.enabled = false;
    guard_config.content_filter.enabled = false;
    
    let guard = Guard::new(guard_config);
    let sanitize_result = guard.sanitize_input(text).await;
    
    match sanitize_result {
        Ok(GuardResult::Clean(_)) => (true, None),
        Ok(GuardResult::Redacted { .. }) => (true, Some("Content contains PII that was redacted".to_string())),
        Ok(GuardResult::Blocked { reason, .. }) => (false, Some(reason)),
        Err(e) => (false, Some(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_sanitize_clean_text() {
        let config = ExtractorConfig::default();
        let (text, info) = sanitize_text("Hello world, this is a test.", &config).await.unwrap();
        assert_eq!(text, "Hello world, this is a test.");
        assert_eq!(info.pii_redacted, 0);
    }
    
    #[tokio::test]
    async fn test_sanitize_with_email() {
        let config = ExtractorConfig::default();
        let (text, info) = sanitize_text("Contact me at test@example.com for details.", &config).await.unwrap();
        assert!(!text.contains("test@example.com"));
        assert!(info.pii_redacted > 0);
    }
    
    #[tokio::test]
    async fn test_check_safety_clean() {
        let config = ExtractorConfig::default();
        let (is_safe, _) = check_safety("This is safe content.", &config).await;
        assert!(is_safe);
    }
}
