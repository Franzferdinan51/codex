//! LM Studio model provider.

use crate::{ModelProviderInfo, WireApi};

const DEFAULT_LMSTUDIO_PORT: u16 = 1234;

/// LM Studio provider configuration.
///
/// LM Studio runs locally and provides an OpenAI-compatible API
/// at http://localhost:1234/v1 by default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LmStudioProvider {
    /// Base URL for the LM Studio API.
    base_url: String,
}

impl LmStudioProvider {
    /// Create a new LM Studio provider with the default localhost URL.
    pub fn new() -> Self {
        Self::with_base_url(format!("http://localhost:{DEFAULT_LMSTUDIO_PORT}/v1"))
    }

    /// Create a new LM Studio provider with a custom base URL.
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    /// Convert this provider into a `ModelProviderInfo` for use in the provider registry.
    pub fn into_model_provider_info(self) -> ModelProviderInfo {
        ModelProviderInfo {
            name: "LM Studio".into(),
            base_url: Some(self.base_url),
            env_key: None,
            env_key_instructions: Some(
                "LM Studio does not require an API key. \
                 Ensure LM Studio is running locally with the API server enabled."
                    .into(),
            ),
            experimental_bearer_token: None,
            auth: None,
            aws: None,
            wire_api: WireApi::Responses,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: None,
            stream_max_retries: None,
            stream_idle_timeout_ms: None,
            websocket_connect_timeout_ms: None,
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }
}

impl Default for LmStudioProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Built-in default LM Studio models.
///
/// These are common models available via LM Studio.
pub const LMSTUDIO_DEFAULT_MODELS: &[&str] = &[
    "lmstudio-community/llama3.2-3b-instruct",
    "lmstudio-community/qwen2.5-7b-instruct",
    "lmstudio-community/mistral-7b-instruct",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lmstudio_provider_default() {
        let provider = LmStudioProvider::new();
        let info = provider.into_model_provider_info();
        assert_eq!(info.name, "LM Studio");
        assert!(info.base_url.as_ref().unwrap().contains("localhost:1234"));
        assert!(!info.requires_openai_auth);
    }

    #[test]
    fn test_lmstudio_provider_custom_url() {
        let provider = LmStudioProvider::with_base_url("http://192.168.1.100:1234/v1");
        let info = provider.into_model_provider_info();
        assert_eq!(info.base_url.as_ref().unwrap(), "http://192.168.1.100:1234/v1");
    }
}