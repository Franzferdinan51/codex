//! Provider infrastructure for DuckHive Codex
//! Supports custom providers like MiniMax, LM Studio, OpenRouter, NVIDIA NIM

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

/// Represents a model from a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub api_type: ProviderType,
    pub base_url: String,
    pub api_key: Option<String>,
    pub default_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProviderType {
    OpenAI,
    OpenRouter,
    MiniMax,
    LMStudio,
    NvidiaNIM,
    Custom,
}

impl ProviderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderType::OpenAI => "openai",
            ProviderType::OpenRouter => "openrouter",
            ProviderType::MiniMax => "minimax",
            ProviderType::LMStudio => "lmstudio",
            ProviderType::NvidiaNIM => "nvidia_nim",
            ProviderType::Custom => "custom",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "openai" => ProviderType::OpenAI,
            "openrouter" => ProviderType::OpenRouter,
            "minimax" => ProviderType::MiniMax,
            "lmstudio" => ProviderType::LMStudio,
            "nvidia_nim" | "nvidia" => ProviderType::NvidiaNIM,
            _ => ProviderType::Custom,
        }
    }
}

/// Provider manager for handling multiple providers
pub struct ProviderManager {
    providers: RwLock<HashMap<String, ProviderConfig>>,
}

impl Default for ProviderManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderManager {
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
        }
    }

    /// Add a new provider
    pub fn add_provider(&self, config: ProviderConfig) -> Result<(), ProviderError> {
        let mut providers = self.providers.write().map_err(|_| ProviderError::LockError)?;
        providers.insert(config.name.clone(), config);
        Ok(())
    }

    /// Remove a provider by name
    pub fn remove_provider(&self, name: &str) -> Result<(), ProviderError> {
        let mut providers = self.providers.write().map_err(|_| ProviderError::LockError)?;
        providers.remove(name);
        Ok(())
    }

    /// Get provider config
    pub fn get_provider(&self, name: &str) -> Option<ProviderConfig> {
        self.providers.read().ok()?.get(name).cloned()
    }

    /// List all providers
    pub fn list_providers(&self) -> Vec<ProviderConfig> {
        self.providers.read().map(|p| p.values().cloned().collect()).unwrap_or_default()
    }

    /// Update provider API key
    pub fn update_api_key(&self, name: &str, api_key: String) -> Result<(), ProviderError> {
        let mut providers = self.providers.write().map_err(|_| ProviderError::LockError)?;
        if let Some(provider) = providers.get_mut(name) {
            provider.api_key = Some(api_key);
            Ok(())
        } else {
            Err(ProviderError::ProviderNotFound(name.to_string()))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),
    #[error("Failed to acquire lock")]
    LockError,
    #[error("API request failed: {0}")]
    ApiError(String),
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

impl ProviderError {
    pub fn api(message: impl Into<String>) -> Self {
        ProviderError::ApiError(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_type_conversion() {
        assert_eq!(ProviderType::from_str("openai"), ProviderType::OpenAI);
        assert_eq!(ProviderType::from_str("OpenRouter"), ProviderType::OpenRouter);
        assert_eq!(ProviderType::from_str("minimax"), ProviderType::MiniMax);
        assert_eq!(ProviderType::from_str("unknown"), ProviderType::Custom);
    }

    #[test]
    fn test_provider_manager_crud() {
        let manager = ProviderManager::new();

        let config = ProviderConfig {
            name: "test".to_string(),
            api_type: ProviderType::OpenAI,
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: Some("sk-test".to_string()),
            default_model: Some("gpt-4".to_string()),
        };

        manager.add_provider(config.clone()).unwrap();
        assert!(manager.get_provider("test").is_some());

        manager.remove_provider("test").unwrap();
        assert!(manager.get_provider("test").is_none());
    }
}