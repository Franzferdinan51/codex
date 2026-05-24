use serde::{Deserialize, Serialize};

/// Custom provider configuration for DuckHive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProviderConfig {
    /// API endpoint URL (e.g., "https://api.minimax.chat/v1")
    pub api_endpoint: String,
    /// API key for authentication
    pub api_key: String,
    /// Optional base path for models endpoint (default: "/v1/models")
    pub models_path: Option<String>,
}

/// Supported provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    /// OpenAI-compatible provider
    OpenAi,
    /// Anthropic provider
    Anthropic,
    /// Google AI (Gemini)
    GoogleAi,
    /// Azure OpenAI
    Azure,
    /// AWS Bedrock
    Bedrock,
    /// Local AI (Ollama)
    LocalAi,
    /// MiniMax AI
    MiniMax,
    /// LM Studio
    LmStudio,
    /// OpenRouter
    OpenRouter,
    /// NVIDIA NIM
    NvidiaNim,
    /// Custom provider with user-specified endpoint
    Custom,
}

impl Default for ProviderType {
    fn default() -> Self {
        Self::OpenAi
    }
}

impl ProviderType {
    /// Returns the default API endpoint for this provider
    pub fn default_endpoint(&self) -> Option<&'static str> {
        match self {
            ProviderType::OpenAi => Some("https://api.openai.com/v1"),
            ProviderType::Anthropic => Some("https://api.anthropic.com/v1"),
            ProviderType::GoogleAi => Some("https://generativelanguage.googleapis.com/v1"),
            ProviderType::Azure => None, // Azure uses custom domain per deployment
            ProviderType::Bedrock => Some("https://bedrock.runtime.us-east-1.amazonaws.com"),
            ProviderType::LocalAi => Some("http://localhost:11434/v1"),
            ProviderType::MiniMax => Some("https://api.minimax.chat/v1"),
            ProviderType::LmStudio => Some("http://localhost:1234/v1"),
            ProviderType::OpenRouter => Some("https://openrouter.ai/api/v1"),
            ProviderType::NvidiaNim => Some("https://integrate.api.nvidia.com/v1"),
            ProviderType::Custom => None,
        }
    }

    /// Returns the default models path for this provider
    pub fn default_models_path(&self) -> &'static str {
        match self {
            ProviderType::OpenAi => "/models",
            ProviderType::Anthropic => "/models",
            ProviderType::GoogleAi => "/models",
            ProviderType::Azure => "/models",
            ProviderType::Bedrock => "/models",
            ProviderType::LocalAi => "/models",
            ProviderType::MiniMax => "/models",
            ProviderType::LmStudio => "/models",
            ProviderType::OpenRouter => "/models",
            ProviderType::NvidiaNim => "/models",
            ProviderType::Custom => "/v1/models",
        }
    }

    /// Returns the display name for this provider
    pub fn display_name(&self) -> &'static str {
        match self {
            ProviderType::OpenAi => "OpenAI",
            ProviderType::Anthropic => "Anthropic",
            ProviderType::GoogleAi => "Google AI",
            ProviderType::Azure => "Azure OpenAI",
            ProviderType::Bedrock => "AWS Bedrock",
            ProviderType::LocalAi => "Local AI (Ollama)",
            ProviderType::MiniMax => "MiniMax",
            ProviderType::LmStudio => "LM Studio",
            ProviderType::OpenRouter => "OpenRouter",
            ProviderType::NvidiaNim => "NVIDIA NIM",
            ProviderType::Custom => "Custom Provider",
        }
    }

    /// Returns true if this provider supports model listing
    pub fn supports_model_listing(&self) -> bool {
        match self {
            ProviderType::Custom => true, // Custom providers may support it
            ProviderType::MiniMax
            | ProviderType::LmStudio
            | ProviderType::OpenRouter
            | ProviderType::NvidiaNim => true,
            _ => false,
        }
    }
}

/// Provider with custom configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProvider {
    pub provider_type: ProviderType,
    pub config: Option<CustomProviderConfig>,
}

impl CustomProvider {
    /// Creates a new custom provider
    pub fn new(provider_type: ProviderType, config: Option<CustomProviderConfig>) -> Self {
        Self {
            provider_type,
            config,
        }
    }

    /// Gets the effective API endpoint
    pub fn api_endpoint(&self) -> Option<String> {
        self.config
            .as_ref()
            .map(|c| c.api_endpoint.clone())
            .or_else(|| self.provider_type.default_endpoint().map(|s| s.to_string()))
    }

    /// Gets the models path
    pub fn models_path(&self) -> String {
        self.config
            .as_ref()
            .and_then(|c| c.models_path.clone())
            .unwrap_or_else(|| self.provider_type.default_models_path().to_string())
    }

    /// Gets the full models endpoint URL
    pub fn models_endpoint(&self) -> Option<String> {
        let base = self.api_endpoint()?;
        let path = self.models_path();
        Some(format!("{}{}", base.trim_end_matches('/'), path))
    }

    /// Gets the API key
    pub fn api_key(&self) -> Option<String> {
        self.config.as_ref().map(|c| c.api_key.clone())
    }
}