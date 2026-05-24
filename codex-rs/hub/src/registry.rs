use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// A skill published on the DuckHive Hub.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HubSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub tags: Vec<String>,
    pub download_url: String,
    pub install_count: u64,
}

/// Remote registry client for the DuckHive Hub.
#[derive(Debug, Clone)]
pub struct HubRegistry {
    client: reqwest::Client,
    base_url: String,
}

impl HubRegistry {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("build reqwest client"),
            base_url: base_url.into(),
        }
    }

    /// Search the hub for skills.
    pub async fn search(&self, query: &str) -> Result<Vec<HubSkill>> {
        let url = format!("{}/api/v1/skills/search?q={}", self.base_url, urlencoding::encode(query));
        tracing::debug!("searching hub: {url}");

        // TODO: when a real hub API exists, replace this with an actual HTTP call.
        // For now return a stub result so the API surface is stable.
        let _ = url;
        tracing::info!("hub search is a stub — returning empty results for '{query}'");
        Ok(Vec::new())
    }

    /// Fetch metadata for a single skill.
    pub async fn get_skill(&self, skill_id: &str) -> Result<HubSkill> {
        let url = format!("{}/api/v1/skills/{skill_id}", self.base_url);
        tracing::debug!("fetching hub skill: {url}");

        // TODO: replace with real HTTP call when hub API exists.
        let _ = url;
        anyhow::bail!("hub skill fetch is not yet implemented")
    }

    /// Download a skill archive to a temporary path.
    pub async fn download_skill(&self, skill_id: &str) -> Result<Vec<u8>> {
        let skill = self.get_skill(skill_id).await?;
        tracing::info!("downloading skill {skill_id} from {}", skill.download_url);

        let bytes = self
            .client
            .get(&skill.download_url)
            .send()
            .await
            .with_context(|| format!("failed to download skill {skill_id}"))?
            .bytes()
            .await
            .with_context(|| format!("failed to read skill {skill_id} response"))?
            .to_vec();

        Ok(bytes)
    }
}
