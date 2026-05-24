#![allow(dead_code)]

pub mod tts;
pub mod wake_word;

use anyhow::Result;

/// Voice subsystem for DuckHive.
///
/// Handles wake-word detection, continuous voice input, and text-to-speech
/// output (currently via ElevenLabs or local synthesis).
#[derive(Debug, Default)]
pub struct VoiceSubsystem {
    wake_word: Option<String>,
    tts_enabled: bool,
}

impl VoiceSubsystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure the wake word that activates the assistant.
    pub fn set_wake_word(&mut self, word: impl Into<String>) {
        self.wake_word = Some(word.into());
    }

    /// Enable or disable TTS output.
    pub fn set_tts_enabled(&mut self, enabled: bool) {
        self.tts_enabled = enabled;
    }

    /// Returns true if the voice subsystem is configured and ready.
    pub fn is_ready(&self) -> bool {
        self.wake_word.is_some() || self.tts_enabled
    }

    /// Start the voice subsystem (wake-word listener + TTS queue).
    ///
    /// This is a stub that logs readiness. In a full implementation it would:
    /// 1. Spawn a microphone audio capture task.
    /// 2. Run the wake-word detection model on the audio stream.
    /// 3. On detection, emit a `WakeEvent` to the main event loop.
    /// 4. Maintain a TTS queue that feeds audio to the system audio output.
    pub async fn run(&self) -> Result<()> {
        tracing::info!("voice subsystem starting");

        if let Some(ref word) = self.wake_word {
            tracing::info!("wake-word listener configured for: {word}");
            // TODO: start wake-word detection loop.
        }

        if self.tts_enabled {
            tracing::info!("TTS output enabled");
            // TODO: start TTS queue processor.
        }

        tracing::info!("voice subsystem started (stubs)");
        Ok(())
    }
}

/// Events emitted by the voice subsystem.
#[derive(Debug, Clone, PartialEq)]
pub enum VoiceEvent {
    /// Wake word was detected.
    WakeDetected { word: String, confidence: f32 },
    /// Speech-to-text result.
    Transcription { text: String, is_final: bool },
    /// TTS playback started.
    TtsStarted { text: String },
    /// TTS playback completed.
    TtsCompleted,
    /// An error occurred.
    Error { message: String },
}
