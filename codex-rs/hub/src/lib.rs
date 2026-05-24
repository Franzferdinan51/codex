#![allow(dead_code)]

pub mod installer;
pub mod registry;

use anyhow::Result;
use registry::HubRegistry;

/// ClawHub-equivalent remote skill marketplace for DuckHive.
///
/// Allows users to discover, install, and manage skills from a remote
/// registry (the "Hub").
#[derive(Debug)]
pub struct HubClient {
    registry: HubRegistry,
}

impl HubClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            registry: HubRegistry::new(base_url),
        }
    }

    /// Search the hub for skills matching a query.
    pub async fn search(&self, query: &str) -> Result<Vec<registry::HubSkill>> {
        self.registry.search(query).await
    }

    /// Install a skill from the hub by ID.
    pub async fn install(&self, skill_id: &str, dest: &std::path::Path) -> Result<()> {
        installer::install_skill(&self.registry, skill_id, dest).await
    }
}
