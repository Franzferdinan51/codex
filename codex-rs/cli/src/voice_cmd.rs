use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage the DuckHive voice subsystem.
#[derive(Debug, Args)]
pub struct VoiceCommand {
    #[command(subcommand)]
    pub subcommand: VoiceSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VoiceSubcommand {
    /// Start the voice subsystem with optional wake-word and TTS.
    Start {
        /// Wake word to listen for (e.g. "DuckHive").
        #[arg(long, value_name = "WORD")]
        wake_word: Option<String>,
        /// Enable text-to-speech output.
        #[arg(long, default_value_t = false)]
        tts: bool,
    },
    /// List supported voice features (stub).
    Status,
}

pub async fn run(cmd: VoiceCommand) -> Result<()> {
    match cmd.subcommand {
        VoiceSubcommand::Start { wake_word, tts } => {
            let mut voice = codex_voice::VoiceSubsystem::new();
            if let Some(word) = wake_word {
                voice.set_wake_word(word);
            }
            voice.set_tts_enabled(tts);
            println!("Voice subsystem starting...");
            voice.run().await?;
            println!("Voice subsystem ready.");
        }
        VoiceSubcommand::Status => {
            println!("Voice features:");
            println!("  • Wake-word detection: stub");
            println!("  • TTS (ElevenLabs): stub");
            println!("  • TTS (OpenAI): stub");
            println!("  • Local TTS: stub");
        }
    }
    Ok(())
}
