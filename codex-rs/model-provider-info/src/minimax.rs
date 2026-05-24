use crate::{ApiKeySource, ModelProvider, ModelProviderKind};

pub const MINIMAX_API_BASE: &str = "https://api.minimax.chat/v1";

/// MiniMax model provider implementation.
pub struct MiniMaxProvider;

impl ModelProvider for MiniMaxProvider {
    fn kind(&self) -> ModelProviderKind {
        ModelProviderKind::MiniMax
    }

    fn api_base(&self) -> &'static str {
        MINIMAX_API_BASE
    }

    fn api_key_source(&self) -> ApiKeySource {
        ApiKeySource::Required("api.minimax.chat".to_string())
    }

    fn supported_models(&self) -> Vec<crate::ModelInfo> {
        vec![
            crate::ModelInfo {
                id: "MiniMax-Text-01".to_string(),
                name: "MiniMax Text 01".to_string(),
                provider_kind: ModelProviderKind::MiniMax,
                supports_functions: true,
                supports_vision: false,
                max_tokens: Some(163_840),
                notes: Some("MiniMax's flagship text model".to_string()),
            },
            crate::ModelInfo {
                id: "MiniMax-Text-01-Moe".to_string(),
                name: "MiniMax Text 01 MoE".to_string(),
                provider_kind: ModelProviderKind::MiniMax,
                supports_functions: true,
                supports_vision: false,
                max_tokens: Some(100_000),
                notes: Some("Mixture-of-Experts variant".to_string()),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimax_provider_kind() {
        let provider = MiniMaxProvider;
        assert_eq!(provider.kind(), ModelProviderKind::MiniMax);
    }

    #[test]
    fn test_minimax_api_base() {
        let provider = MiniMaxProvider;
        assert_eq!(provider.api_base(), "https://api.minimax.chat/v1");
    }

    #[test]
    fn test_minimax_models() {
        let provider = MiniMaxProvider;
        let models = provider.supported_models();
        assert!(!models.is_empty());
        assert_eq!(models[0].id, "MiniMax-Text-01");
    }
}

/// LM Studio model provider implementation.
pub struct LmStudioProvider;

impl ModelProvider for LmStudioProvider {
    fn kind(&self) -> ModelProviderKind {
        ModelProviderKind::LmStudio
    }

    fn api_base(&self) -> &'static str {
        "http://localhost:1234/v1"
    }

    fn api_key_source(&self) -> ApiKeySource {
        ApiKeySource::NotRequired
    }

    fn supported_models(&self) -> Vec<crate::ModelInfo> {
        vec![
            crate::ModelInfo {
                id: "lmstudio-community/llama3.2-3b-instruct".to_string(),
                name: "Llama 3.2 3B".to_string(),
                provider_kind: ModelProviderKind::LmStudio,
                supports_functions: false,
                supports_vision: false,
                max_tokens: Some(8_192),
                notes: Some("Local Llama 3.2 3B instruction model".to_string()),
            },
            crate::ModelInfo {
                id: "lmstudio-community/qwen2.5-7b-instruct".to_string(),
                name: "Qwen 2.5 7B".to_string(),
                provider_kind: ModelProviderKind::LmStudio,
                supports_functions: false,
                supports_vision: false,
                max_tokens: Some(8_192),
                notes: Some("Local Qwen 2.5 7B instruction model".to_string()),
            },
            crate::ModelInfo {
                id: "lmstudio-community/mistral-7b-instruct".to_string(),
                name: "Mistral 7B".to_string(),
                provider_kind: ModelProviderKind::LmStudio,
                supports_functions: false,
                supports_vision: false,
                max_tokens: Some(8_192),
                notes: Some("Local Mistral 7B instruction model".to_string()),
            },
        ]
    }
}

#[cfg(test)]
mod lmstudio_tests {
    use super::*;

    #[test]
    fn test_lmstudio_provider_kind() {
        let provider = LmStudioProvider;
        assert_eq!(provider.kind(), ModelProviderKind::LmStudio);
    }

    #[test]
    fn test_lmstudio_api_base() {
        let provider = LmStudioProvider;
        assert_eq!(provider.api_base(), "http://localhost:1234/v1");
    }

    #[test]
    fn test_lmstudio_models() {
        let provider = LmStudioProvider;
        let models = provider.supported_models();
        assert_eq!(models.len(), 3);
        assert_eq!(models[0].id, "lmstudio-community/llama3.2-3b-instruct");
    }
}