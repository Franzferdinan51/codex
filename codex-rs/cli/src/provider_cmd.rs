//! `codex provider` - Manage custom AI providers
//!
//! Allows adding, configuring, and selecting providers like LM Studio, MiniMax, OpenRouter, and NVIDIA NIM.

use anyhow::Context;
use clap::Parser;
use codex_config::config_toml::ConfigToml;
use codex_model_provider_info::ProviderType;
use std::collections::HashMap;

/// Provider management commands
#[derive(Debug, Parser)]
pub enum ProviderCommand {
    /// List all configured providers
    List,
    /// Add a new provider
    Add {
        /// Provider type (lm-studio, minimax, openrouter, nvidia-nim, custom)
        #[arg(value_name = "TYPE")]
        provider_type: String,
        /// Unique identifier for this provider
        #[arg(long, short)]
        id: String,
        /// Display name for the provider
        #[arg(long, short)]
        name: String,
        /// Base URL for the API endpoint
        #[arg(long)]
        base_url: Option<String>,
        /// API key for authentication
        #[arg(long, short)]
        api_key: Option<String>,
        /// Default model to use
        #[arg(long)]
        default_model: Option<String>,
    },
    /// Remove a provider
    Remove {
        /// Provider ID to remove
        #[arg(value_name = "ID")]
        provider_id: String,
    },
    /// Set the active provider
    SetActive {
        /// Provider ID to activate
        #[arg(value_name = "ID")]
        provider_id: String,
    },
    /// Update provider settings
    Update {
        /// Provider ID to update
        #[arg(value_name = "ID")]
        provider_id: String,
        /// New API key
        #[arg(long)]
        api_key: Option<String>,
        /// New base URL
        #[arg(long)]
        base_url: Option<String>,
        /// New default model
        #[arg(long)]
        default_model: Option<String>,
    },
    /// List available models for a provider
    Models {
        /// Provider ID to fetch models for
        #[arg(value_name = "ID")]
        provider_id: Option<String>,
    },
}

impl ProviderCommand {
    /// Execute the provider command
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            ProviderCommand::List => list_providers().await,
            ProviderCommand::Add {
                provider_type,
                id,
                name,
                base_url,
                api_key,
                default_model,
            } => add_provider(&provider_type, &id, &name, base_url.as_deref(), api_key.as_deref(), default_model.as_deref()).await,
            ProviderCommand::Remove { provider_id } => remove_provider(&provider_id).await,
            ProviderCommand::SetActive { provider_id } => set_active_provider(&provider_id).await,
            ProviderCommand::Update {
                provider_id,
                api_key,
                base_url,
                default_model,
            } => update_provider(&provider_id, api_key.as_deref(), base_url.as_deref(), default_model.as_deref()).await,
            ProviderCommand::Models { provider_id } => list_models(provider_id.as_deref()).await,
        }
    }
}

/// Parse provider type from string
fn parse_provider_type(s: &str) -> anyhow::Result<ProviderType> {
    match s.to_lowercase().as_str() {
        "lm-studio" | "lmstudio" => Ok(ProviderType::LmStudio),
        "minimax" => Ok(ProviderType::MiniMax),
        "openrouter" => Ok(ProviderType::OpenRouter),
        "nvidia-nim" | "nvidia" | "nim" => Ok(ProviderType::NvidiaNim),
        "custom" => Ok(ProviderType::Custom),
        _ => anyhow::bail!(
            "Unknown provider type: {s}\nValid types: lm-studio, minimax, openrouter, nvidia-nim, custom"
        ),
    }
}

async fn list_providers() -> anyhow::Result<()> {
    let config = ConfigToml::load_default().context("Failed to load config")?;
    let provider_config = config.provider_config().unwrap_or_default();

    if provider_config.providers.is_empty() {
        println!("No providers configured. Use `codex provider add` to add one.");
        return Ok(());
    }

    println!("Configured Providers:\n");
    for (id, provider) in &provider_config.providers {
        let active_marker = if provider_config.active_provider_id.as_deref() == Some(id) {
            " [ACTIVE]"
        } else {
            ""
        };
        println!("  {id}{active_marker}");
        println!("    Name: {}", provider.name);
        println!("    Type: {}", provider.settings.get("type").map(|s| s.as_str()).unwrap_or("unknown"));
        if let Some(base_url) = provider.base_url.strip_suffix("/v1") {
            println!("    Base URL: {base_url}");
        } else {
            println!("    Base URL: {}", provider.base_url);
        }
        if let Some(model) = &provider.default_model {
            println!("    Default Model: {model}");
        }
        if provider.api_key.is_some() {
            println!("    API Key: [configured]");
        }
        println!();
    }

    Ok(())
}

async fn add_provider(
    provider_type: &str,
    id: &str,
    name: &str,
    base_url: Option<&str>,
    api_key: Option<&str>,
    default_model: Option<&str>,
) -> anyhow::Result<()> {
    let ptype = parse_provider_type(provider_type)?;
    let default_url = ptype.default_base_url().unwrap_or("");
    let final_base_url = base_url.unwrap_or(default_url);

    if final_base_url.is_empty() {
        anyhow::bail!("Base URL is required for custom providers. Use --base-url");
    }

    let mut config = ConfigToml::load_default().context("Failed to load config")?;
    let mut provider_config = config.provider_config().unwrap_or_default();

    // Build settings map
    let mut settings = HashMap::new();
    settings.insert("type".to_string(), provider_type.to_string());

    // Create provider
    let mut provider = codex_model_provider_info::Provider::new(id, name, final_base_url);
    if let Some(key) = api_key {
        provider = provider.with_api_key(key);
    }
    if let Some(model) = default_model {
        provider = provider.with_default_model(model);
    }
    provider.settings = settings;

    provider_config.add_provider(provider);
    config.set_provider_config(provider_config);

    ConfigToml::save(&config).context("Failed to save config")?;

    println!("Added provider '{id}' ({name}) of type {provider_type}");
    println!("  Base URL: {final_base_url}");
    if api_key.is_some() {
        println!("  API Key: [configured]");
    }
    if default_model.is_some() {
        println!("  Default Model: {default_model}");
    }

    Ok(())
}

async fn remove_provider(provider_id: &str) -> anyhow::Result<()> {
    let mut config = ConfigToml::load_default().context("Failed to load config")?;
    let mut provider_config = config.provider_config().unwrap_or_default();

    match provider_config.remove_provider(provider_id) {
        Some(removed) => {
            if provider_config.active_provider_id.as_deref() == Some(provider_id) {
                provider_config.active_provider_id = None;
            }
            config.set_provider_config(provider_config);
            ConfigToml::save(&config).context("Failed to save config")?;
            println!("Removed provider '{provider_id}' ({})", removed.name);
        }
        None => {
            anyhow::bail!("Provider '{provider_id}' not found");
        }
    }

    Ok(())
}

async fn set_active_provider(provider_id: &str) -> anyhow::Result<()> {
    let mut config = ConfigToml::load_default().context("Failed to load config")?;
    let mut provider_config = config.provider_config().unwrap_or_default();

    if provider_config.set_active(provider_id) {
        config.set_provider_config(provider_config.clone());
        ConfigToml::save(&config).context("Failed to save config")?;
        println!("Active provider set to '{provider_id}'");
    } else {
        anyhow::bail!("Provider '{provider_id}' not found");
    }

    Ok(())
}

async fn update_provider(
    provider_id: &str,
    api_key: Option<&str>,
    base_url: Option<&str>,
    default_model: Option<&str>,
) -> anyhow::Result<()> {
    let mut config = ConfigToml::load_default().context("Failed to load config")?;
    let mut provider_config = config.provider_config().unwrap_or_default();

    let provider = provider_config
        .providers
        .get_mut(provider_id)
        .ok_or_else(|| anyhow::anyhow!("Provider '{provider_id}' not found"))?;

    let mut updated = false;
    if let Some(key) = api_key {
        provider.api_key = Some(key.to_string());
        updated = true;
        println!("Updated API key for '{provider_id}'");
    }
    if let Some(url) = base_url {
        provider.base_url = url.to_string();
        updated = true;
        println!("Updated base URL for '{provider_id}' to '{url}'");
    }
    if let Some(model) = default_model {
        provider.default_model = Some(model.to_string());
        updated = true;
        println!("Updated default model for '{provider_id}' to '{model}'");
    }

    if updated {
        config.set_provider_config(provider_config);
        ConfigToml::save(&config).context("Failed to save config")?;
    } else {
        println!("No changes specified. Use --api-key, --base-url, or --default-model");
    }

    Ok(())
}

async fn list_models(provider_id: Option<&str>) -> anyhow::Result<()> {
    let config = ConfigToml::load_default().context("Failed to load config")?;
    let provider_config = config.provider_config().unwrap_or_default();

    let provider = match provider_id {
        Some(id) => provider_config
            .get_provider(id)
            .ok_or_else(|| anyhow::anyhow!("Provider '{id}' not found"))?,
        None => provider_config
            .active_provider()
            .ok_or_else(|| anyhow::anyhow!("No active provider set. Specify a provider ID or set one with `codex provider set-active`"))?,
    };

    println!("Fetching models from {}...", provider.name);

    // Fetch models from the provider's models endpoint
    let client = reqwest::Client::new();
    let mut request = client.get(format!("{}/models", provider.base_url.trim_end_matches('/v1')));

    if let Some(ref api_key) = provider.api_key {
        request = request.header("Authorization", format!("Bearer {api_key}"));
    }

    match request.send().await {
        Ok(response) => {
            if response.status().is_success() {
                #[derive(serde::Deserialize)]
                struct ModelsResponse {
                    data: Vec<ModelInfo>,
                }
                #[derive(serde::Deserialize)]
                struct ModelInfo {
                    id: String,
                    #[serde(default)]
                    name: Option<String>,
                    #[serde(default)]
                    description: Option<String>,
                }

                match response.json::<ModelsResponse>().await {
                    Ok(models) => {
                        println!("\nAvailable models:\n");
                        for model in models.data {
                            println!("  {}", model.id);
                            if let Some(name) = model.name {
                                if name != model.id {
                                    println!("    Name: {name}");
                                }
                            }
                        }
                    }
                    Err(_) => {
                        println!("Response parsed but unable to read models. Provider may use different format.");
                    }
                }
            } else {
                println!("Failed to fetch models: HTTP {}", response.status());
            }
        }
        Err(e) => {
            println!("Failed to connect to provider: {e}");
            println!("Make sure the provider is running and the base URL is correct.");
        }
    }

    Ok(())
}