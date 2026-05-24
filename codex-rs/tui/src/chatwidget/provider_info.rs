use crate::ChatWidget;
use codex_tui_styles::Stylize;
use ratatui::text::Line;

/// Known provider metadata for display purposes.
struct KnownProvider {
    name: &'static str,
    models: &'static [&'static str],
    endpoint: &'static str,
}

/// All providers the user can configure.
const KNOWN_PROVIDERS: &[KnownProvider] = &[
    KnownProvider {
        name: "Anthropic",
        models: &["claude-sonnet-4-20250514", "claude-opus-4-20250514", "claude-3.5-haiku"],
        endpoint: "https://api.anthropic.com/v1",
    },
    KnownProvider {
        name: "OpenAI",
        models: &["gpt-4.1", "gpt-4.1-mini", "o3", "o4-mini"],
        endpoint: "https://api.openai.com/v1",
    },
    KnownProvider {
        name: "OpenRouter",
        models: &["anthropic/claude-sonnet-4", "openai/gpt-4.1", "google/gemini-2.5-pro"],
        endpoint: "https://openrouter.ai/api/v1",
    },
    KnownProvider {
        name: "MiniMax",
        models: &["MiniMax-Text-01", "MiniMax-M1"],
        endpoint: "https://api.minimax.chat/v1",
    },
    KnownProvider {
        name: "LM Studio",
        models: &["local-model"],
        endpoint: "http://localhost:1234/v1",
    },
    KnownProvider {
        name: "Ollama",
        models: &["llama3", "codellama", "mistral"],
        endpoint: "http://localhost:11434/v1",
    },
    KnownProvider {
        name: "Groq",
        models: &["llama-3.3-70b", "mixtral-8x7b"],
        endpoint: "https://api.groq.com/openai/v1",
    },
    KnownProvider {
        name: "Together AI",
        models: &["meta-llama/Llama-3.3-70B", "mistralai/Mixtral-8x7B"],
        endpoint: "https://api.together.xyz/v1",
    },
    KnownProvider {
        name: "Mistral",
        models: &["mistral-large-latest", "mistral-small-latest"],
        endpoint: "https://api.mistral.ai/v1",
    },
    KnownProvider {
        name: "Google / Gemini",
        models: &["gemini-2.5-pro", "gemini-2.5-flash"],
        endpoint: "https://generativelanguage.googleapis.com/v1beta",
    },
    KnownProvider {
        name: "DeepSeek",
        models: &["deepseek-chat", "deepseek-reasoner"],
        endpoint: "https://api.deepseek.com/v1",
    },
    KnownProvider {
        name: "xAI / Grok",
        models: &["grok-3", "grok-3-mini"],
        endpoint: "https://api.x.ai/v1",
    },
];

/// Detect a human-friendly provider label from the base URL.
fn detect_provider_label(url: &str) -> &'static str {
    let lower = url.to_lowercase();
    if lower.contains("anthropic") {
        "Anthropic"
    } else if lower.contains("openai") {
        "OpenAI"
    } else if lower.contains("openrouter") {
        "OpenRouter"
    } else if lower.contains("minimax") {
        "MiniMax"
    } else if lower.contains("lmstudio") || lower.contains("lm-studio") {
        "LM Studio"
    } else if lower.contains("ollama") {
        "Ollama"
    } else if lower.contains("groq") {
        "Groq"
    } else if lower.contains("together") {
        "Together AI"
    } else if lower.contains("mistral") {
        "Mistral"
    } else if lower.contains("google") || lower.contains("gemini") {
        "Google / Gemini"
    } else if lower.contains("azure") {
        "Azure OpenAI"
    } else if lower.contains("deepseek") {
        "DeepSeek"
    } else if lower.contains("xai") || lower.contains("grok") {
        "xAI / Grok"
    } else {
        "Custom"
    }
}

impl ChatWidget {
    pub fn add_provider_output(&mut self) {
        let mut lines: Vec<Line<'static>> = Vec::new();

        // Header
        lines.push(Line::from(vec![
            "DuckHive Provider Configuration".bold().cyan(),
        ]));
        lines.push(Line::from(""));

        // Section: Current Provider
        lines.push("Active Provider".bold().into());
        lines.push(Line::from(""));

        let provider_name = self.config.provider.as_deref().unwrap_or("default");
        lines.push(Line::from(vec![
            "  Name:      ".dim(),
            provider_name.cyan().bold(),
        ]));

        // Auto-detect provider from base URL
        let base_url = self.config.base_url.as_deref().unwrap_or("");
        let detected = if base_url.is_empty() {
            "(auto-detect)".dim().to_string()
        } else {
            detect_provider_label(base_url).cyan().to_string()
        };
        lines.push(Line::from(vec![
            "  Detected:  ".dim(),
            Line::from(detected),
        ]));

        // Section: Connection
        lines.push(Line::from(""));
        lines.push("Connection".bold().into());
        lines.push(Line::from(""));

        // Current model
        let model_name = self.config.model.as_deref().unwrap_or("default");
        lines.push(Line::from(vec![
            "  Model:     ".dim(),
            model_name.cyan(),
        ]));

        // API base URL
        let display_url = if base_url.is_empty() {
            "(not configured)".dim().to_string()
        } else {
            base_url.to_string()
        };
        lines.push(Line::from(vec![
            "  Base URL:  ".dim(),
            display_url.into(),
        ]));

        // API Key status (masked for security)
        let api_key_status = match self.config.api_key.as_deref() {
            Some(key) if !key.is_empty() => {
                let masked = if key.len() > 8 {
                    format!("{}...{}", &key[..4], &key[key.len()-4..])
                } else {
                    "****".to_string()
                };
                masked.green().to_string()
            }
            _ => "(not set)".red().to_string(),
        };
        lines.push(Line::from(vec![
            "  API Key:   ".dim(),
            Line::from(api_key_status),
        ]));

        // Organization (if configured)
        if let Some(ref org) = self.config.organization {
            lines.push(Line::from(vec![
                "  Org:       ".dim(),
                org.as_str().into(),
            ]));
        }

        // Connection status
        let status = if self.last_error.is_some() {
            "error".red()
        } else {
            "connected".green()
        };
        lines.push(Line::from(vec![
            "  Status:    ".dim(),
            status,
        ]));

        if let Some(ref err) = self.last_error {
            lines.push(Line::from(vec![
                "  Error:     ".dim(),
                err.as_str().red(),
            ]));
        }

        // Section: Model Parameters
        lines.push(Line::from(""));
        lines.push("Model Parameters".bold().into());
        lines.push(Line::from(""));

        // Show common parameters with defaults
        if let Some(temp) = self.config.temperature {
            lines.push(Line::from(vec![
                "  Temperature:    ".dim(),
                format!("{:.2}", temp).cyan(),
            ]));
        } else {
            lines.push(Line::from(vec![
                "  Temperature:    ".dim(),
                "default".dim(),
            ]));
        }

        if let Some(max_tokens) = self.config.max_tokens {
            lines.push(Line::from(vec![
                "  Max tokens:     ".dim(),
                format!("{max_tokens}").cyan(),
            ]));
        } else {
            lines.push(Line::from(vec![
                "  Max tokens:     ".dim(),
                "default".dim(),
            ]));
        }

        // Show custom parameters from config
        if let Some(ref params) = self.config.model_parameters {
            for (key, value) in params {
                lines.push(Line::from(vec![
                    format!("  {key}:  ").dim(),
                    format!("{value}").into(),
                ]));
            }
        }

        // Section: All Known Providers
        lines.push(Line::from(""));
        lines.push("Available Providers".bold().into());
        lines.push(Line::from(""));

        let active_detected = detect_provider_label(base_url);

        for provider in KNOWN_PROVIDERS {
            let is_active = provider.name == active_detected
                || provider.name.eq_ignore_ascii_case(provider_name);

            let status_marker = if is_active {
                " [active]".green().bold()
            } else {
                "".into()
            };

            lines.push(Line::from(vec![
                "  ".into(),
                provider.name.bold(),
                status_marker,
            ]));

            lines.push(Line::from(vec![
                "    Endpoint: ".dim(),
                provider.endpoint.dim(),
            ]));

            let models_str = provider.models.join(", ");
            lines.push(Line::from(vec![
                "    Models:   ".dim(),
                models_str.into(),
            ]));

            lines.push(Line::from(""));
        }

        // Section: System Info
        lines.push("System".bold().into());
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            "  Runtime:   ".dim(),
            "DuckHive (mimo-v2.5-pro)".cyan(),
        ]));
        lines.push(Line::from(vec![
            "  Version:   ".dim(),
            env!("CARGO_PKG_VERSION").into(),
        ]));

        // Usage hints
        lines.push(Line::from(""));
        lines.push("Tips".bold().dim());
        lines.push(Line::from(vec![
            "  ".into(),
            "Use /model to switch models".dim(),
        ]));
        lines.push(Line::from(vec![
            "  ".into(),
            "Configure providers in config.toml".dim(),
        ]));

        self.add_output(lines);
    }
}
