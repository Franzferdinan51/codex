use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Custom provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub provider_type: ProviderType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    OpenRouter,
    LMStudio,
    MiniMax,
    NvidiaNIM,
    Custom,
}

impl ProviderConfig {
    pub fn new(name: String, base_url: String, provider_type: ProviderType) -> Self {
        Self {
            name,
            base_url,
            api_key: None,
            provider_type,
        }
    }

    /// Fetch models from provider's API
    pub async fn fetch_models(&self) -> Result<Vec<ModelInfo>, String> {
        let client = reqwest::Client::new();
        let mut request = client.get(format!("{}/models", self.base_url));

        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await.map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            return Err(format!("HTTP {}", response.status()));
        }

        let models: Vec<ModelInfo> = response.json().await.map_err(|e| e.to_string())?;
        Ok(models)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderStore {
    providers: Vec<ProviderConfig>,
}

impl Default for ProviderStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderStore {
    pub fn new() -> Self {
        Self { providers: Vec::new() }
    }

    pub fn add_provider(&mut self, provider: ProviderConfig) {
        self.providers.push(provider);
    }

    pub fn get_providers(&self) -> &[ProviderConfig] {
        &self.providers
    }

    pub fn remove_provider(&mut self, name: &str) -> bool {
        let len_before = self.providers.len();
        self.providers.retain(|p| p.name != name);
        self.providers.len() < len_before
    }
}

/// Global provider store
pub type SharedProviderStore = Arc<RwLock<ProviderStore>>;

pub fn create_provider_store() -> SharedProviderStore {
    Arc::new(RwLock::new(ProviderStore::new()))
}