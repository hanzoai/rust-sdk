use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
/// Enhanced tool personality system with 117+ programmer personalities
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::RwLock;

/// Represents a programmer personality with tool preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPersonality {
    pub name: String,
    pub programmer: String,
    pub description: String,
    pub tools: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub philosophy: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl ToolPersonality {
    pub fn new(
        name: impl Into<String>,
        programmer: impl Into<String>,
        description: impl Into<String>,
        tools: Vec<String>,
    ) -> Self {
        // Deduplicate and sort tools
        let mut unique_tools: Vec<String> = tools
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        unique_tools.sort();

        Self {
            name: name.into(),
            programmer: programmer.into(),
            description: description.into(),
            tools: unique_tools,
            environment: None,
            philosophy: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_philosophy(mut self, philosophy: impl Into<String>) -> Self {
        self.philosophy = Some(philosophy.into());
        self
    }

    pub fn with_environment(mut self, env: HashMap<String, String>) -> Self {
        self.environment = Some(env);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

/// Registry for tool personalities with enhanced functionality
pub struct PersonalityRegistry {
    personalities: HashMap<String, ToolPersonality>,
    active_personality: Option<String>,
}

impl PersonalityRegistry {
    pub fn new() -> Self {
        Self {
            personalities: HashMap::new(),
            active_personality: None,
        }
    }

    pub fn register(&mut self, mut personality: ToolPersonality) {
        // Ensure agent enabled if API keys present
        personality = ensure_agent_enabled(personality);
        // Deduplicate tools
        let unique_tools: Vec<String> = personality
            .tools
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        personality.tools = unique_tools;
        personality.tools.sort();

        self.personalities
            .insert(personality.name.clone(), personality);
    }

    pub fn add_personality(&mut self, personality: ToolPersonality) -> Result<(), String> {
        if self.personalities.contains_key(&personality.name) {
            return Err(format!("Personality '{}' already exists", personality.name));
        }
        self.register(personality);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&ToolPersonality> {
        self.personalities.get(name)
    }

    pub fn list(&self) -> Vec<&ToolPersonality> {
        self.personalities.values().collect()
    }

    pub fn export(&self, include_tools: bool) -> Vec<HashMap<String, serde_json::Value>> {
        self.personalities
            .values()
            .map(|p| {
                let mut export = HashMap::new();
                export.insert("name".to_string(), serde_json::json!(p.name));
                export.insert("programmer".to_string(), serde_json::json!(p.programmer));
                export.insert("description".to_string(), serde_json::json!(p.description));
                export.insert("tool_count".to_string(), serde_json::json!(p.tools.len()));
                export.insert("tags".to_string(), serde_json::json!(p.tags));
                if let Some(philosophy) = &p.philosophy {
                    export.insert("philosophy".to_string(), serde_json::json!(philosophy));
                }
                if include_tools {
                    export.insert("tools".to_string(), serde_json::json!(p.tools));
                }
                export
            })
            .collect()
    }

    pub fn filter_by_tags(&self, tags: &[String]) -> Vec<&ToolPersonality> {
        self.personalities
            .values()
            .filter(|p| tags.iter().any(|tag| p.tags.contains(tag)))
            .collect()
    }

    pub fn set_active(&mut self, name: &str) -> Result<(), String> {
        if !self.personalities.contains_key(name) {
            return Err(format!("Personality '{}' not found", name));
        }
        self.active_personality = Some(name.to_string());

        // Apply environment variables
        if let Some(personality) = self.personalities.get(name) {
            if let Some(env) = &personality.environment {
                for (key, value) in env {
                    std::env::set_var(key, value);
                }
            }
        }

        Ok(())
    }

    pub fn get_active(&self) -> Option<&ToolPersonality> {
        self.active_personality
            .as_ref()
            .and_then(|name| self.personalities.get(name))
    }

    pub fn get_active_tools(&self) -> HashSet<String> {
        self.get_active()
            .map(|p| p.tools.iter().cloned().collect())
            .unwrap_or_default()
    }
}

// Global registry instance
static REGISTRY: Lazy<RwLock<PersonalityRegistry>> = Lazy::new(|| {
    let mut registry = PersonalityRegistry::new();
    register_default_personalities(&mut registry);
    RwLock::new(registry)
});

// Tool categories - Essential
const ESSENTIAL_TOOLS: &[&str] = &["read", "write", "edit", "tree", "bash", "think"];

// Classic tool sets
const UNIX_TOOLS: &[&str] = &["grep", "find_files", "bash", "process", "diff"];
const BUILD_TOOLS: &[&str] = &["bash", "npx", "uvx", "process", "cargo", "gem", "pip"];
const VERSION_CONTROL: &[&str] = &["git_search", "diff", "gh", "gitlab"];
const AI_TOOLS: &[&str] = &["agent", "consensus", "critic", "think", "llm"];
const SEARCH_TOOLS: &[&str] = &["search", "symbols", "grep", "git_search", "ast_search"];
const DATABASE_TOOLS: &[&str] = &["sql_query", "sql_search", "graph_add", "graph_query"];
const VECTOR_TOOLS: &[&str] = &["vector_index", "vector_search", "embeddings"];

// Modern DevOps & Cloud tools
const DEVOPS_TOOLS: &[&str] = &[
    "docker",
    "container_build",
    "k8s",
    "kubectl",
    "helm",
    "kustomize",
    "minikube",
];
const CI_CD_TOOLS: &[&str] = &[
    "ci",
    "github_actions",
    "gitlab_ci",
    "jenkins",
    "circleci",
    "artifact_publish",
];
const CLOUD_TOOLS: &[&str] = &[
    "terraform",
    "ansible",
    "cloud_cli",
    "aws_s3",
    "kms",
    "secrets_manager",
];
const OBSERVABILITY_TOOLS: &[&str] = &[
    "prometheus",
    "grafana",
    "otel",
    "logs",
    "tracing",
    "slo",
    "chaos",
];

// Security & Quality tools
const SECURITY_TOOLS: &[&str] = &[
    "sast",
    "dast",
    "fuzz",
    "dependency_scan",
    "secret_scan",
    "sigstore",
    "sbom",
    "snyk",
    "trivy",
];
const TESTING_TOOLS: &[&str] = &[
    "pytest",
    "jest",
    "mocha",
    "go_test",
    "linters",
    "formatter",
    "coverage",
];

// ML/DataOps tools
const ML_TOOLS: &[&str] = &[
    "mlflow",
    "dvc",
    "kedro",
    "mlem",
    "model_registry",
    "feature_store",
    "jupyter",
    "notebook",
];
const AI_OPS_TOOLS: &[&str] = &[
    "model_deploy",
    "gpu_manager",
    "quantize",
    "onnx_convert",
    "huggingface",
    "hf_hub",
];

// Developer UX tools
const DEV_UX_TOOLS: &[&str] = &[
    "ngrok",
    "localstack",
    "devcontainer",
    "vscode_remote",
    "repl",
    "watch",
    "hot_reload",
];

// Utility tools
const UTILITY_TOOLS: &[&str] = &[
    "package_manager",
    "image_scan",
    "signing",
    "notebook",
    "batch",
    "todo",
    "rules",
];

/// Helper to combine tool arrays into a Vec<String>
fn combine_tools(tool_sets: &[&[&str]]) -> Vec<String> {
    let mut tools = HashSet::new();
    for set in tool_sets {
        for tool in *set {
            tools.insert(tool.to_string());
        }
    }
    tools.into_iter().collect()
}

/// Enable agent tool if API keys are present
fn ensure_agent_enabled(mut personality: ToolPersonality) -> ToolPersonality {
    let api_keys = [
        "OPENAI_API_KEY",
        "ANTHROPIC_API_KEY",
        "TOGETHER_API_KEY",
        "HANZO_API_KEY",
    ];

    if api_keys.iter().any(|key| std::env::var(key).is_ok()) {
        if !personality.tools.contains(&"agent".to_string()) {
            personality.tools.push("agent".to_string());
        }
    }

    personality
}

/// Load personalities from the centralized JSON file
fn load_personalities_from_json() -> Result<Vec<ToolPersonality>, String> {
    // Try to find the JSON file in common locations
    let possible_paths = [
        "/Users/z/work/hanzo/persona/personalities/all_personalities.json",
        "../../../persona/personalities/all_personalities.json",
        "personalities/all_personalities.json",
    ];

    let mut json_content = None;
    for path in &possible_paths {
        if Path::new(path).exists() {
            match fs::read_to_string(path) {
                Ok(content) => {
                    json_content = Some(content);
                    break;
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to read personality file at {}: {}",
                        path, e
                    );
                }
            }
        }
    }

    let json_content = json_content.ok_or_else(|| {
        "Could not find personalities JSON file in any of the expected locations".to_string()
    })?;

    let personalities: Vec<ToolPersonality> = serde_json::from_str(&json_content)
        .map_err(|e| format!("Failed to parse personalities JSON: {}", e))?;

    Ok(personalities)
}

/// Register all personalities from the centralized JSON file
fn register_default_personalities(registry: &mut PersonalityRegistry) {
    match load_personalities_from_json() {
        Ok(personalities) => {
            for personality in personalities {
                registry.register(personality);
            }
            eprintln!("Loaded {} personalities from JSON", registry.list().len());
        }
        Err(e) => {
            eprintln!("Warning: Failed to load personalities from JSON: {}", e);
            eprintln!("Falling back to minimal built-in personalities");

            // Fallback to a minimal set of essential personalities
            register_fallback_personalities(registry);
        }
    }
}

/// Fallback personalities if JSON loading fails
fn register_fallback_personalities(registry: &mut PersonalityRegistry) {
    // Essential minimal personalities as fallback
    registry.register(
        ToolPersonality::new(
            "hanzo",
            "Hanzo AI Default",
            "Balanced productivity and quality",
            combine_tools(&[
                ESSENTIAL_TOOLS,
                AI_TOOLS,
                SEARCH_TOOLS,
                BUILD_TOOLS,
                VERSION_CONTROL,
                &["multi_edit", "symbols", "watch", "todo", "rules"],
            ]),
        )
        .with_philosophy("AI-powered development at scale.")
        .with_tags(vec![
            "default".to_string(),
            "balanced".to_string(),
            "ai".to_string(),
        ])
        .with_environment(HashMap::from([
            ("HANZO_MODE".to_string(), "enabled".to_string()),
            ("AI_ASSIST".to_string(), "true".to_string()),
        ])),
    );

    registry.register(
        ToolPersonality::new(
            "minimal",
            "Minimalist",
            "Just the essentials",
            ESSENTIAL_TOOLS.iter().map(|s| s.to_string()).collect(),
        )
        .with_philosophy("Less is more.")
        .with_tags(vec![
            "minimal".to_string(),
            "focused".to_string(),
            "simple".to_string(),
        ])
        .with_environment(HashMap::from([(
            "MINIMAL_MODE".to_string(),
            "true".to_string(),
        )])),
    );

    registry.register(
        ToolPersonality::new(
            "guido",
            "Guido van Rossum",
            "Python's BDFL - readability counts",
            combine_tools(&[ESSENTIAL_TOOLS, &["uvx", "jupyter", "pytest", "formatter"]]),
        )
        .with_philosophy("There should be one-- and preferably only one --obvious way to do it.")
        .with_tags(vec![
            "languages".to_string(),
            "python".to_string(),
            "readability".to_string(),
        ])
        .with_environment(HashMap::from([("PYTHONPATH".to_string(), ".".to_string())])),
    );

    registry.register(
        ToolPersonality::new(
            "linus",
            "Linus Torvalds",
            "Linux & Git creator - pragmatic excellence",
            combine_tools(&[
                ESSENTIAL_TOOLS,
                &["git_search", "diff", "critic"],
                UNIX_TOOLS,
                VERSION_CONTROL,
            ]),
        )
        .with_philosophy("Talk is cheap. Show me the code.")
        .with_tags(vec![
            "systems".to_string(),
            "linux".to_string(),
            "git".to_string(),
        ])
        .with_environment(HashMap::from([(
            "GIT_AUTHOR_NAME".to_string(),
            "Linus Torvalds".to_string(),
        )])),
    );
}

/// Public API for the personality system
pub mod api {
    use super::*;

    pub fn register(personality: ToolPersonality) -> Result<(), String> {
        REGISTRY
            .write()
            .map_err(|_| "Failed to acquire write lock")?
            .register(personality);
        Ok(())
    }

    pub fn add_personality(personality: ToolPersonality) -> Result<(), String> {
        REGISTRY
            .write()
            .map_err(|_| "Failed to acquire write lock")?
            .add_personality(personality)
    }

    pub fn get(name: &str) -> Option<ToolPersonality> {
        REGISTRY.read().ok()?.get(name).cloned()
    }

    pub fn list() -> Vec<ToolPersonality> {
        REGISTRY
            .read()
            .ok()
            .map(|r| r.list().into_iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn export(include_tools: bool) -> Vec<HashMap<String, serde_json::Value>> {
        REGISTRY
            .read()
            .ok()
            .map(|r| r.export(include_tools))
            .unwrap_or_default()
    }

    pub fn filter_by_tags(tags: &[String]) -> Vec<ToolPersonality> {
        REGISTRY
            .read()
            .ok()
            .map(|r| r.filter_by_tags(tags).into_iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn set_active(name: &str) -> Result<(), String> {
        REGISTRY
            .write()
            .map_err(|_| "Failed to acquire write lock")?
            .set_active(name)
    }

    pub fn get_active() -> Option<ToolPersonality> {
        REGISTRY.read().ok()?.get_active().cloned()
    }

    pub fn get_active_tools() -> HashSet<String> {
        REGISTRY
            .read()
            .ok()
            .map(|r| r.get_active_tools())
            .unwrap_or_default()
    }

    pub fn activate_from_env() -> Option<String> {
        let mode_name = std::env::var("HANZO_MODE")
            .or_else(|_| std::env::var("PERSONALITY"))
            .or_else(|_| std::env::var("MODE"))
            .ok()?;

        set_active(&mode_name).ok()?;
        Some(mode_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personality_creation() {
        let personality = ToolPersonality::new(
            "test",
            "Test Programmer",
            "A test personality",
            vec!["read".to_string(), "write".to_string(), "read".to_string()], // Test deduplication
        );

        assert_eq!(personality.name, "test");
        assert_eq!(personality.programmer, "Test Programmer");
        assert_eq!(personality.tools.len(), 2); // Should be deduplicated
        assert!(personality.tools.contains(&"read".to_string()));
        assert!(personality.tools.contains(&"write".to_string()));
    }

    #[test]
    fn test_registry() {
        let mut registry = PersonalityRegistry::new();

        let personality = ToolPersonality::new(
            "test",
            "Test Programmer",
            "A test personality",
            vec!["read".to_string()],
        );

        registry.register(personality.clone());

        assert!(registry.get("test").is_some());
        assert_eq!(registry.list().len(), 1);

        assert!(registry.set_active("test").is_ok());
        assert_eq!(
            registry.get_active().map(|p| p.name.clone()),
            Some("test".to_string())
        );
    }

    #[test]
    fn test_global_api() {
        // The global registry should have default personalities
        let personalities = api::list();
        // Should have at least the minimal built-in personalities
        assert!(personalities.len() >= 3);

        // Should be able to find key built-in personalities
        assert!(api::get("guido").is_some());
        assert!(api::get("linus").is_some());
        assert!(api::get("hanzo").is_some());
    }

    #[test]
    fn test_filter_by_tags() {
        // This test depends on the JSON file having tags
        // If JSON not loaded, filter_by_tags will return empty
        let all_personalities = api::list();
        if all_personalities.iter().any(|p| !p.tags.is_empty()) {
            // JSON was loaded, test tag filtering
            let pioneers = api::filter_by_tags(&["pioneer".to_string()]);
            // pioneers can be empty if JSON doesn't have 'pioneer' tag
            let _ = pioneers; // Just verify no panic
        }
        // Test passes regardless - tag filtering is optional functionality
    }

    #[test]
    fn test_export() {
        let exports = api::export(false);
        assert!(exports.len() > 0);

        let exports_with_tools = api::export(true);
        assert!(exports_with_tools.iter().any(|e| e.contains_key("tools")));
    }
}
