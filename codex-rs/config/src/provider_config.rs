use serde::{Deserialize, Serialize};

/// Custom provider configuration for DuckHive-Codex
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConfig {
    /// Provider name (e.g., "lm-studio", "minimax", "openrouter", "nvidia-nim")
    pub name: String,
    /// API base URL
    pub base_url: String,
    /// API key (optional for some providers like LM Studio)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Default model to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model: Option<String>,
    /// Whether this provider is enabled
    #[serde(default)]
    pub enabled: bool,
}

impl ProviderConfig {
    /// Create a new provider configuration
    pub fn new(name: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            base_url: base_url.into(),
            api_key: None,
            default_model: None,
            enabled: true,
        }
    }

    /// Set the API key
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the default model
    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = Some(model.into());
        self
    }
}

/// Provider type enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderType {
    /// LM Studio - local model server
    LmStudio,
    /// MiniMax AI
    MiniMax,
    /// OpenRouter - unified API for multiple providers
    OpenRouter,
    /// NVIDIA NIM - NVIDIA's model inference service
    NvidiaNim,
    /// Custom provider
    Custom,
}

impl ProviderType {
    /// Get the default base URL for this provider type
    pub fn default_base_url(self) -> Option<&'static str> {
        match self {
            ProviderType::LmStudio => Some("http://localhost:1234"),
            ProviderType::MiniMax => Some("https://api.minimax.chat/v1"),
            ProviderType::OpenRouter => Some("https://openrouter.ai/api/v1"),
            ProviderType::NvidiaNim => Some("https://integrate.api.nvidia.com/v1"),
            ProviderType::Custom => None,
        }
    }

    /// Get the provider name
    pub fn name(self) -> &'static str {
        match self {
            ProviderType::LmStudio => "lm-studio",
            ProviderType::MiniMax => "minimax",
            ProviderType::OpenRouter => "openrouter",
            ProviderType::NvidiaNim => "nvidia-nim",
            ProviderType::Custom => "custom",
        }
    }
}

/// Model info from a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_length: Option<u32>,
}

/// Provider models response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderModelsResponse {
    pub models: Vec<ModelInfo>,
}

/// Provider manager - handles provider CRUD operations and model fetching
#[derive(Debug, Clone)]
pub struct ProviderManager {
    pub providers: std::collections::HashMap<String, ProviderConfig>,
}

impl ProviderManager {
    /// Create a new provider manager
    pub fn new() -> Self {
        Self {
            providers: std::collections::HashMap::new(),
        }
    }

    /// Add a provider configuration
    pub fn add_provider(&mut self, config: ProviderConfig) {
        self.providers.insert(config.name.clone(), config);
    }

    /// Remove a provider by name
    pub fn remove_provider(&mut self, name: &str) {
        self.providers.remove(name);
    }

    /// Get a provider by name
    pub fn get_provider(&self, name: &str) -> Option<&ProviderConfig> {
        self.providers.get(name)
    }

    /// Get provider names as a Vec
    pub fn provider_names(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }

    /// Get models for a provider (cached)
    pub fn get_models(&self, provider_name: &str) -> Option<Vec<ModelInfo>> {
        self.providers
            .get(provider_name)
            .and_then(|p| p.default_model.clone())
            .map(|m| vec![ModelInfo {
                id: m.clone(),
                name: m,
                description: None,
                context_length: None,
            }])
    }

    /// Set the default model for a provider
    pub fn set_default_model(&mut self, provider_name: &str, model: String) {
        if let Some(config) = self.providers.get_mut(provider_name) {
            config.default_model = Some(model);
        }
    }
}

impl Default for ProviderManager {
    fn default() -> Self {
        Self::new()
    }
}