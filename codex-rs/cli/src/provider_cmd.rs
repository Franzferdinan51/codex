//! `codex provider` – Manage custom model providers interactively.

use clap::Parser;
use codex_config::provider_edit::add_model_provider;
use codex_config::provider_edit::load_model_providers;
use codex_config::provider_edit::remove_model_provider;
use codex_core::config::find_codex_home;
use codex_model_provider_info::ModelProviderInfo;
use codex_model_provider_info::WireApi;
use dialoguer::Confirm;
use dialoguer::Input;
use dialoguer::Select;

/// Known third-party providers with sensible defaults.
/// Sourced from https://github.com/Gitlawb/openclaude docs/advanced-setup.md
const KNOWN_PROVIDERS: &[(&str, &str, &str, &str)] = &[
    // Cloud providers
    ("openai", "OpenAI", "https://api.openai.com/v1", "OPENAI_API_KEY"),
    (
        "deepseek",
        "DeepSeek",
        "https://api.deepseek.com/v1",
        "DEEPSEEK_API_KEY",
    ),
    (
        "openrouter",
        "OpenRouter",
        "https://openrouter.ai/api/v1",
        "OPENROUTER_API_KEY",
    ),
    ("groq", "Groq", "https://api.groq.com/openai/v1", "GROQ_API_KEY"),
    (
        "together",
        "Together AI",
        "https://api.together.xyz/v1",
        "TOGETHER_API_KEY",
    ),
    (
        "mistral",
        "Mistral",
        "https://api.mistral.ai/v1",
        "MISTRAL_API_KEY",
    ),
    (
        "gemini",
        "Gemini (OpenAI-compatible)",
        "https://generativelanguage.googleapis.com/v1beta/openai/",
        "GEMINI_API_KEY",
    ),
    (
        "xiaomi-mimo",
        "Xiaomi MiMo",
        "https://api.xiaomimimo.com/v1",
        "MIMO_API_KEY",
    ),
    (
        "gitlawb-opengateway",
        "Gitlawb Opengateway",
        "https://opengateway.gitlawb.com/v1",
        "OPENAI_API_KEY",
    ),
    (
        "azure-openai",
        "Azure OpenAI",
        "",
        "AZURE_OPENAI_API_KEY",
    ),
    (
        "minimax",
        "MiniMax",
        "https://api.minimax.chat/v1",
        "MINIMAX_API_KEY",
    ),
    (
        "nvidia-nim",
        "NVIDIA NIM",
        "https://integrate.api.nvidia.com/v1",
        "NVIDIA_API_KEY",
    ),
    // Local providers (no API key required)
    (
        "ollama",
        "Ollama",
        "http://localhost:11434/v1",
        "",
    ),
    (
        "lmstudio",
        "LM Studio",
        "http://localhost:1234/v1",
        "",
    ),
    (
        "atomic-chat",
        "Atomic Chat",
        "http://127.0.0.1:1337/v1",
        "",
    ),
    ("custom", "Custom", "", ""),
];

/// Provider management commands.
#[derive(Debug, Parser)]
pub enum ProviderCommand {
    /// List all custom providers configured in config.toml.
    List,
    /// Add a new custom provider.
    Add {
        /// Provider identifier (e.g. openrouter).
        #[arg(value_name = "ID", required = false)]
        provider_id: Option<String>,
    },
    /// Remove a custom provider.
    Remove {
        /// Provider identifier to remove.
        #[arg(value_name = "ID", required = false)]
        provider_id: Option<String>,
    },
    /// Fetch and list models from a provider.
    Models {
        /// Provider identifier. Uses the `model_provider` default if omitted.
        #[arg(value_name = "ID", required = false)]
        provider_id: Option<String>,
    },
}

impl ProviderCommand {
    pub async fn run(self) -> anyhow::Result<()> {
        match self {
            ProviderCommand::List => list_providers().await,
            ProviderCommand::Add { provider_id } => add_provider_interactive(provider_id).await,
            ProviderCommand::Remove { provider_id } => remove_provider_interactive(provider_id).await,
            ProviderCommand::Models { provider_id } => list_models(provider_id).await,
        }
    }
}

async fn list_providers() -> anyhow::Result<()> {
    let codex_home = find_codex_home()?;
    let providers = load_model_providers(codex_home.as_path())?;

    if providers.is_empty() {
        println!("No custom providers configured.");
        println!("Run `codex provider add` to add one.");
        return Ok(());
    }

    println!("Custom model providers:\n");
    let mut ids: Vec<_> = providers.keys().collect();
    ids.sort();
    for id in ids {
        let info = &providers[id];
        println!("  {id}");
        if !info.name.is_empty() {
            println!("    name: {}", info.name);
        }
        if let Some(url) = &info.base_url {
            println!("    base_url: {url}");
        }
        if let Some(key) = &info.env_key {
            println!("    env_key: {key}");
        }
        if info.requires_openai_auth {
            println!("    requires_openai_auth: true");
        }
        println!();
    }

    Ok(())
}

async fn add_provider_interactive(provider_id: Option<String>) -> anyhow::Result<()> {
    let codex_home = find_codex_home()?;

    // If a provider ID was passed non-interactively, skip the wizard.
    let provider_id = match provider_id {
        Some(id) => id,
        None => {
            println!("Add a custom model provider\n");

            let names: Vec<String> = KNOWN_PROVIDERS
                .iter()
                .map(|(_, name, _, _)| name.to_string())
                .collect();

            let selection = Select::new()
                .with_prompt("Choose a provider template")
                .items(&names)
                .default(0)
                .interact()?;

            KNOWN_PROVIDERS[selection].0.to_string()
        }
    };

    let template = KNOWN_PROVIDERS
        .iter()
        .find(|(id, _, _, _)| *id == provider_id)
        .map(|(_, name, url, key)| (name.to_string(), url.to_string(), key.to_string()))
        .unwrap_or_else(|| {
            (
                provider_id.clone(),
                String::new(),
                format!("{}_API_KEY", provider_id.to_uppercase().replace('-', "_")),
            )
        });

    // Interactive prompts for the remaining fields.
    let name: String = Input::new()
        .with_prompt("Display name")
        .default(template.0)
        .interact_text()?;

    let base_url: String = Input::new()
        .with_prompt("Base URL")
        .default(template.1)
        .allow_empty(true)
        .interact_text()?;

    let env_key: String = Input::new()
        .with_prompt("Environment variable for API key")
        .default(template.2)
        .allow_empty(true)
        .interact_text()?;

    let supports_websockets = Confirm::new()
        .with_prompt("Does this provider support WebSockets?")
        .default(false)
        .interact()?;

    let requires_openai_auth = Confirm::new()
        .with_prompt("Does this provider require OpenAI-style authentication?")
        .default(false)
        .interact()?;

    let info = ModelProviderInfo {
        name,
        base_url: if base_url.is_empty() { None } else { Some(base_url) },
        env_key: if env_key.is_empty() { None } else { Some(env_key) },
        env_key_instructions: None,
        experimental_bearer_token: None,
        auth: None,
        aws: None,
        wire_api: WireApi::Responses,
        query_params: None,
        http_headers: None,
        env_http_headers: None,
        request_max_retries: None,
        stream_max_retries: None,
        stream_idle_timeout_ms: None,
        websocket_connect_timeout_ms: None,
        requires_openai_auth,
        supports_websockets,
    };

    add_model_provider(codex_home.as_path(), &provider_id, &info)?;

    println!("\nAdded provider '{provider_id}'.");
    if !env_key.is_empty() {
        println!("Set your API key: export {env_key}=<your-key>");
    }

    Ok(())
}

async fn remove_provider_interactive(provider_id: Option<String>) -> anyhow::Result<()> {
    let codex_home = find_codex_home()?;
    let providers = load_model_providers(codex_home.as_path())?;

    if providers.is_empty() {
        println!("No custom providers to remove.");
        return Ok(());
    }

    let provider_id = match provider_id {
        Some(id) => id,
        None => {
            let ids: Vec<String> = providers.keys().cloned().collect();
            let selection = Select::new()
                .with_prompt("Select a provider to remove")
                .items(&ids)
                .interact()?;
            ids[selection].clone()
        }
    };

    if !providers.contains_key(&provider_id) {
        anyhow::bail!("Provider '{provider_id}' not found in config.");
    }

    if Confirm::new()
        .with_prompt(format!("Remove provider '{provider_id}'?"))
        .default(false)
        .interact()?
    {
        remove_model_provider(codex_home.as_path(), &provider_id)?;
        println!("Removed provider '{provider_id}'.");
    } else {
        println!("Cancelled.");
    }

    Ok(())
}

async fn list_models(provider_id: Option<String>) -> anyhow::Result<()> {
    let codex_home = find_codex_home()?;
    let providers = load_model_providers(codex_home.as_path())?;

    let provider_id = match provider_id {
        Some(id) => id,
        None => {
            let ids: Vec<String> = providers.keys().cloned().collect();
            if ids.is_empty() {
                anyhow::bail!(
                    "No custom providers configured. Specify a provider ID or add one first."
                );
            }
            let selection = Select::new()
                .with_prompt("Select a provider")
                .items(&ids)
                .interact()?;
            ids[selection].clone()
        }
    };

    let info = providers
        .get(&provider_id)
        .ok_or_else(|| anyhow::anyhow!("Provider '{provider_id}' not found in config."))?
        .clone();

    let base_url = info.base_url.as_deref().unwrap_or("");
    if base_url.is_empty() {
        anyhow::bail!("Provider '{provider_id}' has no base_url configured.");
    }

    let url = format!("{base_url}/models");
    println!("Fetching models from {url} ...\n");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let mut request = client.get(&url);

    if let Some(env_key) = &info.env_key {
        if let Ok(api_key) = std::env::var(env_key) {
            request = request.header("Authorization", format!("Bearer {api_key}"));
        }
    }

    match request.send().await {
        Ok(response) => {
            if response.status().is_success() {
                #[derive(serde::Deserialize)]
                struct ModelsResponse {
                    data: Vec<ModelEntry>,
                }
                #[derive(serde::Deserialize)]
                struct ModelEntry {
                    id: String,
                    #[serde(default)]
                    object: Option<String>,
                }

                match response.json::<ModelsResponse>().await {
                    Ok(models) => {
                        if models.data.is_empty() {
                            println!("No models returned.");
                        } else {
                            println!("Available models:\n");
                            for model in models.data {
                                println!("  {}", model.id);
                            }
                        }
                    }
                    Err(err) => {
                        println!("Failed to parse response: {err}");
                    }
                }
            } else {
                println!("Provider returned HTTP {}", response.status());
            }
        }
        Err(err) => {
            println!("Failed to connect: {err}");
        }
    }

    Ok(())
}
