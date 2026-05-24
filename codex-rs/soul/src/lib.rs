pub mod soul_md;
pub mod tools_md;

use std::path::Path;

use anyhow::Result;
use codex_utils_absolute_path::AbsolutePathBuf;

use crate::soul_md::SoulConfig;
use crate::tools_md::ToolsConfig;

/// Extract YAML frontmatter from a markdown-style file that starts with
/// `---` and ends with the next `---`.
pub(crate) fn extract_frontmatter(raw: &str) -> Option<String> {
    let mut lines = raw.lines();
    if !matches!(lines.next(), Some(line) if line.trim() == "---") {
        return None;
    }

    let mut frontmatter_lines: Vec<&str> = Vec::new();
    let mut found_closing = false;
    for line in lines.by_ref() {
        if line.trim() == "---" {
            found_closing = true;
            break;
        }
        frontmatter_lines.push(line);
    }

    if frontmatter_lines.is_empty() || !found_closing {
        return None;
    }

    Some(frontmatter_lines.join("\n"))
}

/// Parsed workspace configuration for a DuckHive agent workspace.
/// Combines SOUL.md (personality / identity) and TOOLS.md (tool declarations).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorkspaceConfig {
    pub soul: Option<SoulConfig>,
    pub tools: Option<ToolsConfig>,
}

impl WorkspaceConfig {
    /// Load workspace config from a directory that may contain `SOUL.md`
    /// and/or `TOOLS.md`.
    pub async fn load_from_dir(dir: &Path) -> Result<Self> {
        let soul = soul_md::load_from_dir(dir).await?;
        let tools = tools_md::load_from_dir(dir).await?;
        Ok(Self { soul, tools })
    }

    /// Returns true if the workspace has any configuration loaded.
    pub fn is_empty(&self) -> bool {
        self.soul.is_none() && self.tools.is_none()
    }
}

/// Discover workspace configs by walking up from `cwd` looking for
/// `.duckhive/` directories that contain SOUL.md / TOOLS.md.
pub async fn discover_workspace_configs(
    cwd: &AbsolutePathBuf,
) -> Result<Vec<(std::path::PathBuf, WorkspaceConfig)>> {
    let mut configs = Vec::new();
    for ancestor in cwd.as_path().ancestors() {
        let duckhive_dir = ancestor.join(".duckhive");
        if duckhive_dir.is_dir() {
            let config = WorkspaceConfig::load_from_dir(&duckhive_dir).await?;
            if !config.is_empty() {
                configs.push((duckhive_dir, config));
            }
        }
    }
    configs.reverse();
    Ok(configs)
}
