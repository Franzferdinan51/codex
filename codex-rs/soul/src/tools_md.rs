use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

const TOOLS_FILENAME: &str = "TOOLS.md";

/// Parsed TOOLS.md workspace tool declarations.
/// Defines workspace-specific tools that the agent can invoke.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct ToolsConfig {
    pub tools: Vec<ToolDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ToolDeclaration {
    pub name: String,
    pub description: String,
    pub command: Option<String>,
    pub script: Option<String>,
    #[serde(default)]
    pub parameters: HashMap<String, ToolParameter>,
    #[serde(default)]
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ToolParameter {
    pub description: Option<String>,
    pub required: bool,
    pub param_type: Option<String>,
}

/// Load `TOOLS.md` from `dir` if it exists.
pub async fn load_from_dir(dir: &Path) -> Result<Option<ToolsConfig>> {
    let path = dir.join(TOOLS_FILENAME);
    if !path.is_file() {
        return Ok(None);
    }

    let raw = tokio::fs::read_to_string(&path)
        .await
        .with_context(|| format!("failed to read {TOOLS_FILENAME}"))?;

    let config = parse_tools_md(&raw)?;
    Ok(Some(config))
}

fn parse_tools_md(raw: &str) -> Result<ToolsConfig> {
    let frontmatter = crate::extract_frontmatter(raw).unwrap_or_default();
    let config: ToolsConfig = if frontmatter.trim().is_empty() {
        ToolsConfig::default()
    } else {
        serde_yaml::from_str(&frontmatter)
            .with_context(|| "invalid TOOLS.md frontmatter YAML")?
    };
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tools_md() {
        let raw = r#"---
tools:
  - name: run_tests
    description: Run the project test suite
    command: cargo test
    requires_confirmation: false
  - name: deploy
    description: Deploy to production
    command: ./scripts/deploy.sh
    requires_confirmation: true
---
"#;
        let config = parse_tools_md(raw).expect("parse");
        assert_eq!(config.tools.len(), 2);
        assert_eq!(config.tools[0].name, "run_tests");
        assert!(!config.tools[0].requires_confirmation);
        assert!(config.tools[1].requires_confirmation);
    }
}
