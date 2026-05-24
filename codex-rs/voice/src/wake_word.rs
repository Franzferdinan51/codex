use crate::VoiceEvent;

/// Wake-word detection engine stub.
///
/// In a full implementation this would load a lightweight keyword-spotting
/// model (e.g., Porcupine, Whisper, or a custom ONNX model) and run it on
/// a live audio stream.
#[derive(Debug, Default)]
pub struct WakeWordDetector {
    keyword: Option<String>,
}

impl WakeWordDetector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_keyword(&mut self, keyword: impl Into<String>) {
        self.keyword = Some(keyword.into());
    }

    /// Start listening for the wake word.
    ///
    /// Returns an async stream of `VoiceEvent`s. Stub implementation
    /// returns an empty stream.
    pub async fn listen(&self) -> anyhow::Result<Vec<VoiceEvent>> {
        tracing::info!(
            "wake-word detector listening for: {}",
            self.keyword.as_deref().unwrap_or("(none)")
        );
        // TODO: implement real audio capture + keyword spotting.
        Ok(Vec::new())
    }
}
