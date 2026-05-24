use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

const SOUL_FILENAME: &str = "SOUL.md";

/// Parsed SOUL.md workspace configuration.
/// Defines the agent's personality, identity, and behavioral rules for a
/// given workspace.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct SoulConfig {
    pub name: Option<String>,
    pub description: Option<String>,
    pub personality: Option<String>,
    pub rules: Vec<String>,
    pub voice: Option<VoiceConfig>,
    pub greeting: Option<String>,
    #[serde(default)]
    pub allow_file_access: bool,
    #[serde(default)]
    pub allow_shell_access: bool,
    #[serde(default)]
    pub allow_web_access: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct VoiceConfig {
    pub wake_word: Option<String>,
    pub tts_provider: Option<String>,
    pub tts_voice_id: Option<String>,
}

/// Load `SOUL.md` from `dir` if it exists.
pub async fn load_from_dir(dir: &Path) -> Result<Option<SoulConfig>> {
    let path = dir.join(SOUL_FILENAME);
    if !path.is_file() {
        return Ok(None);
    }

    let raw = tokio::fs::read_to_string(&path)
        .await
        .with_context(|| format!("failed to read {SOUL_FILENAME}"))?;

    let config = parse_soul_md(&raw)?;
    Ok(Some(config))
}

/// Parse SOUL.md frontmatter and body.
fn parse_soul_md(raw: &str) -> Result<SoulConfig> {
    let frontmatter = crate::extract_frontmatter(raw).unwrap_or_default();
    let mut config: SoulConfig = if frontmatter.trim().is_empty() {
        SoulConfig::default()
    } else {
        serde_yaml::from_str(&frontmatter)
            .with_context(|| "invalid SOUL.md frontmatter YAML")?
    };

    // The body after frontmatter becomes the personality text.
    let body = extract_body(raw);
    if config.personality.is_none() && !body.trim().is_empty() {
        config.personality = Some(body.trim().to_string());
    }

    Ok(config)
}

fn extract_body(raw: &str) -> String {
    let mut lines = raw.lines();
    if matches!(lines.next(), Some(line) if line.trim() == "---") {
        for line in lines.by_ref() {
            if line.trim() == "---" {
                break;
            }
        }
    }
    lines.collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_soul_md() {
        let raw = r#"---
name: Code Mentor
personality: friendly and patient
allow_file_access: true
---
You are a helpful coding mentor. Always explain your reasoning.
"#;
        let config = parse_soul_md(raw).expect("parse");
        assert_eq!(config.name, Some("Code Mentor".to_string()));
        assert_eq!(config.personality, Some("friendly and patient".to_string()));
        assert!(config.allow_file_access);
    }

    #[test]
    fn parse_body_only_soul_md() {
        let raw = "You are a general assistant.";
        let config = parse_soul_md(raw).expect("parse");
        assert_eq!(config.name, None);
        assert_eq!(config.personality, Some("You are a general assistant.".to_string()));
    }
}
