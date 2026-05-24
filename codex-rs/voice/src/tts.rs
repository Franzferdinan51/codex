use serde::{Deserialize, Serialize};

/// Text-to-speech provider configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TtsConfig {
    pub provider: TtsProvider,
    pub voice_id: Option<String>,
    pub model_id: Option<String>,
    pub api_key_env: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TtsProvider {
    Elevenlabs,
    Local,
    Openai,
}

/// TTS synthesis engine stub.
///
/// In a full implementation this would queue text fragments, stream them
/// to the configured TTS provider, and feed the resulting audio to the
/// system audio output.
#[derive(Debug, Default)]
pub struct TtsEngine {
    config: Option<TtsConfig>,
    queue: Vec<String>,
}

impl TtsEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn configure(&mut self, config: TtsConfig) {
        tracing::info!("TTS configured with provider: {:?}", config.provider);
        self.config = Some(config);
    }

    /// Enqueue text to be spoken.
    pub fn speak(&mut self, text: impl Into<String>) {
        let text = text.into();
        tracing::debug!("TTS enqueue: {text}");
        self.queue.push(text);
    }

    /// Process the TTS queue. Stub: drains the queue and logs.
    pub async fn flush(&mut self) -> anyhow::Result<()> {
        while let Some(text) = self.queue.pop() {
            tracing::info!("TTS would speak: {text}");
            // TODO: call actual TTS provider API and play audio.
        }
        Ok(())
    }
}
