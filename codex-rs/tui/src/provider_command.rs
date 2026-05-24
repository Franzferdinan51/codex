use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

use crate::chatwidget::provider_info::ProviderInfo;
use crate::status_bar::StatusBar;

/// Provider details for display
#[derive(Debug, Clone)]
pub struct ProviderDetails {
    pub name: &'static str,
    pub description: &'static str,
    pub default_model: &'static str,
    pub api_base: &'static str,
    pub requires_api_key: bool,
}

/// All known providers with their details
pub const KNOWN_PROVIDERS: &[ProviderDetails] = &[
    ProviderDetails {
        name: "openai",
        description: "OpenAI GPT models (GPT-4o, GPT-4, etc.)",
        default_model: "gpt-4o",
        api_base: "https://api.openai.com/v1",
        requires_api_key: true,
    },
    ProviderDetails {
        name: "anthropic",
        description: "Anthropic Claude models (Claude 3.5, Claude 3, etc.)",
        default_model: "claude-3-5-sonnet-20241022",
        api_base: "https://api.anthropic.com",
        requires_api_key: true,
    },
    ProviderDetails {
        name: "azure",
        description: "Microsoft Azure OpenAI Service",
        default_model: "gpt-4o",
        api_base: "https://{resource}.openai.azure.com",
        requires_api_key: true,
    },
    ProviderDetails {
        name: "openrouter",
        description: "OpenRouter - access multiple providers through one API",
        default_model: "anthropic/claude-3.5-sonnet",
        api_base: "https://openrouter.ai/api/v1",
        requires_api_key: true,
    },
    ProviderDetails {
        name: "lmstudio",
        description: "LM Studio - local model inference server",
        default_model: "local-model",
        api_base: "http://localhost:1234/v1",
        requires_api_key: false,
    },
    ProviderDetails {
        name: "minimax",
        description: "MiniMax - AI language models",
        default_model: "MiniMax-Text-01",
        api_base: "https://api.minimax.chat/v1",
        requires_api_key: true,
    },
    ProviderDetails {
        name: "nim",
        description: "NVIDIA NIM - GPU-optimized model inference",
        default_model: "meta/llama-3.1-8b-instruct",
        api_base: "https://integrate.api.nvidia.com/v1",
        requires_api_key: true,
    },
    ProviderDetails {
        name: "ollama",
        description: "Ollama - run open source models locally",
        default_model: "llama3.2",
        api_base: "http://localhost:11434/v1",
        requires_api_key: false,
    },
];

/// Format provider list for display in the chat
pub fn format_provider_list(current_provider: &str, current_model: &str) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    lines.push(Line::from(vec![
        "  ".into(),
        "Available Providers".bold().cyan(),
    ]));
    lines.push(Line::from(vec![
        "  ".into(),
        "─────────────────".dim(),
    ]));

    for provider in KNOWN_PROVIDERS {
        let is_active = provider.name == current_provider;
        let marker = if is_active {
            Span::from(" ● ").green().bold()
        } else {
            "    ".into()
        };

        let name_span = if is_active {
            Span::from(provider.name).green().bold()
        } else {
            Span::from(provider.name).cyan()
        };

        let desc_span = Span::from(provider.description).dim();

        lines.push(Line::from(vec![
            "  ".into(),
            marker,
            name_span,
            " - ".into(),
            desc_span,
        ]));

        if is_active {
            lines.push(Line::from(vec![
                "       ".into(),
                "Model: ".dim(),
                Span::from(current_model).yellow(),
            ]));
            lines.push(Line::from(vec![
                "       ".into(),
                "Base URL: ".dim(),
                Span::from(provider.api_base).dim(),
            ]));
        }
    }

    lines.push(Line::from(vec!["".into()]));
    lines.push(Line::from(vec![
        "  ".into(),
        "Usage:".bold(),
    ]));
    lines.push(Line::from(vec![
        "    ".into(),
        "/provider".cyan(),
        "              ".into(),
        "Show this help".dim(),
    ]));
    lines.push(Line::from(vec![
        "    ".into(),
        "/provider <name>".cyan(),
        "       ".into(),
        "Switch to provider".dim(),
    ]));
    lines.push(Line::from(vec![
        "    ".into(),
        "/provider list".cyan(),
        "        ".into(),
        "List available providers".dim(),
    ]));
    lines.push(Line::from(vec![
        "    ".into(),
        "/provider info".cyan(),
        "        ".into(),
        "Show current provider details".dim(),
    ]));
    lines.push(Line::from(vec![
        "    ".into(),
        "/provider models".cyan(),
        "      ".into(),
        "List models for current provider".dim(),
    ]));

    lines
}

/// Format detailed info for a specific provider
pub fn format_provider_info(
    provider_name: &str,
    current_model: &str,
    api_base: &str,
    has_api_key: bool,
) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    let provider = KNOWN_PROVIDERS
        .iter()
        .find(|p| p.name == provider_name);

    lines.push(Line::from(vec![
        "  ".into(),
        "Provider Details".bold().cyan(),
    ]));
    lines.push(Line::from(vec![
        "  ".into(),
        "───────────────".dim(),
    ]));

    if let Some(p) = provider {
        lines.push(Line::from(vec![
            "  ".into(),
            "Name: ".dim(),
            Span::from(p.name).green().bold(),
        ]));
        lines.push(Line::from(vec![
            "  ".into(),
            "Description: ".dim(),
            Span::from(p.description),
        ]));
        lines.push(Line::from(vec![
            "  ".into(),
            "API Base: ".dim(),
            Span::from(api_base).dim(),
        ]));
        lines.push(Line::from(vec![
            "  ".into(),
            "Default Model: ".dim(),
            Span::from(p.default_model).yellow(),
        ]));
        lines.push(Line::from(vec![
            "  ".into(),
            "Requires API Key: ".dim(),
            if has_api_key {
                Span::from("Yes ✓").green()
            } else if p.requires_api_key {
                Span::from("Not set ✗").red()
            } else {
                Span::from("No (local)").green()
            },
        ]));
    } else {
        lines.push(Line::from(vec![
            "  ".into(),
            "Provider: ".dim(),
            Span::from(provider_name).yellow(),
        ]));
        lines.push(Line::from(vec![
            "  ".into(),
            "Status: ".dim(),
            Span::from("Custom configuration"),
        ]));
        lines.push(Line::from(vec![
            "  ".into(),
            "API Base: ".dim(),
            Span::from(api_base).dim(),
        ]));
        lines.push(Line::from(vec![
            "  ".into(),
            "Model: ".dim(),
            Span::from(current_model).yellow(),
        ]));
    }

    lines
}

/// Format model list (placeholder - will be populated from provider API)
pub fn format_model_list(provider_name: &str) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    let provider = KNOWN_PROVIDERS
        .iter()
        .find(|p| p.name == provider_name);

    lines.push(Line::from(vec![
        "  ".into(),
        format!("Models for {provider_name}").bold().cyan(),
    ]));
    lines.push(Line::from(vec![
        "  ".into(),
        "─────────────────────".dim(),
    ]));

    if let Some(p) = provider {
        match p.name {
            "openai" => {
                let models = ["gpt-4o", "gpt-4o-mini", "gpt-4-turbo", "gpt-4", "gpt-3.5-turbo", "o1-preview", "o1-mini"];
                for model in models {
                    let is_default = model == p.default_model;
                    let marker = if is_default { " ● " } else { "   " };
                    let span = if is_default {
                        Span::from(model).green().bold()
                    } else {
                        Span::from(model)
                    };
                    lines.push(Line::from(vec!["  ".into(), marker.dim(), span]));
                }
            }
            "anthropic" => {
                let models = ["claude-3-5-sonnet-20241022", "claude-3-5-haiku-20241022", "claude-3-opus-20240229", "claude-3-sonnet-20240229", "claude-3-haiku-20240307"];
                for model in models {
                    let is_default = model == p.default_model;
                    let marker = if is_default { " ● " } else { "   " };
                    let span = if is_default {
                        Span::from(model).green().bold()
                    } else {
                        Span::from(model)
                    };
                    lines.push(Line::from(vec!["  ".into(), marker.dim(), span]));
                }
            }
            "lmstudio" => {
                lines.push(Line::from(vec![
                    "  ".into(),
                    "   ".into(),
                    "Any model loaded in LM Studio".dim(),
                ]));
                lines.push(Line::from(vec![
                    "  ".into(),
                    "   ".into(),
                    "Default: local-model".yellow(),
                ]));
            }
            "ollama" => {
                let models = ["llama3.2", "llama3.1", "llama3", "mistral", "codellama", "phi3", "gemma2"];
                for model in models {
                    let is_default = model == p.default_model;
                    let marker = if is_default { " ● " } else { "   " };
                    let span = if is_default {
                        Span::from(model).green().bold()
                    } else {
                        Span::from(model)
                    };
                    lines.push(Line::from(vec!["  ".into(), marker.dim(), span]));
                }
            }
            _ => {
                lines.push(Line::from(vec![
                    "  ".into(),
                    "   ".into(),
                    format!("Default: {}", p.default_model).yellow(),
                ]));
            }
        }
    } else {
        lines.push(Line::from(vec![
            "  ".into(),
            "   ".into(),
            "Custom provider - models depend on configuration".dim(),
        ]));
    }

    lines.push(Line::from(vec!["".into()]));
    lines.push(Line::from(vec![
        "  ".into(),
        "Tip: ".dim(),
        "Use ".into(),
        "/model <name>".cyan(),
        " to switch models".into(),
    ]));

    lines
}

/// Handle /provider command with arguments
pub fn handle_provider_command(
    args: &str,
    current_provider: &str,
    current_model: &str,
    api_base: &str,
    has_api_key: bool,
) -> Vec<Line<'static>> {
    let args = args.trim();

    if args.is_empty() || args == "help" {
        return format_provider_list(current_provider, current_model);
    }

    if args == "list" {
        return format_provider_list(current_provider, current_model);
    }

    if args == "info" {
        return format_provider_info(current_provider, current_model, api_base, has_api_key);
    }

    if args == "models" {
        return format_model_list(current_provider);
    }

    // Check if it's a provider name to switch to
    let target_provider = args.to_lowercase();
    if KNOWN_PROVIDERS.iter().any(|p| p.name == target_provider) {
        vec![
            Line::from(vec![
                "  ".into(),
                "Switching to provider: ".into(),
                Span::from(&target_provider).green().bold(),
            ]),
            Line::from(vec![
                "  ".into(),
                "Note: ".dim(),
                "Use ".into(),
                format!("/model {}", target_provider).cyan(),
                " after switching to set a specific model".into(),
            ]),
            Line::from(vec![
                "  ".into(),
                "Hint: ".dim(),
                "Make sure to set the API key in config if required".dim(),
            ]),
        ]
    } else {
        vec![
            Line::from(vec![
                "  ".into(),
                "Unknown provider: ".into(),
                Span::from(&target_provider).red(),
            ]),
            Line::from(vec![
                "  ".into(),
                "Use ".into(),
                "/provider list".cyan(),
                " to see available providers".into(),
            ]),
        ]
    }
}
