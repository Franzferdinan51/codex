use std::path::Path;

use anyhow::{Context, Result};
use tracing::info;

use crate::registry::HubRegistry;

/// Install a skill from the hub into the given destination directory.
///
/// The destination is expected to be a `skills/` directory under the
/// user's DuckHive home or a workspace `.duckhive/skills/` folder.
pub async fn install_skill(
    registry: &HubRegistry,
    skill_id: &str,
    dest: &Path,
) -> Result<()> {
    info!("installing skill {skill_id} into {}", dest.display());

    let archive = registry
        .download_skill(skill_id)
        .await
        .with_context(|| format!("failed to download skill {skill_id}"))?;

    let skill_dir = dest.join(skill_id);
    tokio::fs::create_dir_all(&skill_dir)
        .await
        .with_context(|| format!("failed to create skill dir {}", skill_dir.display()))?;

    // TODO: decompress archive (zip / tar.gz) into skill_dir.
    // For now write a marker so the directory is valid.
    let marker = skill_dir.join(".duckhive-skill");
    tokio::fs::write(&marker, b"installed from hub")
        .await
        .with_context(|| format!("failed to write skill marker {}", marker.display()))?;

    info!("skill {skill_id} installed at {}", skill_dir.display());
    Ok(())
}
