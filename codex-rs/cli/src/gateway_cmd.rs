use anyhow::Result;
use clap::{Args, Subcommand};
use std::collections::HashSet;

/// Manage the DuckHive multi-channel gateway.
#[derive(Debug, Args)]
pub struct GatewayCommand {
    #[command(subcommand)]
    pub subcommand: GatewaySubcommand,
}

#[derive(Debug, Subcommand)]
pub enum GatewaySubcommand {
    /// Start the gateway daemon with the specified channel adapters.
    Start {
        /// Channel adapters to enable (comma-separated).
        /// Supported: slack, discord, telegram, whatsapp, irc, matrix, signal, imessage
        #[arg(long, value_delimiter = ',', value_name = "ADAPTERS")]
        adapters: Vec<String>,
    },
    /// List all supported channel adapters.
    ListAdapters,
    /// Show gateway status (stub).
    Status,
}

pub async fn run(cmd: GatewayCommand) -> Result<()> {
    match cmd.subcommand {
        GatewaySubcommand::Start { adapters } => {
            let router = codex_gateway::router::AgentRouter::new();
            let (mut daemon, _tx) = codex_gateway::GatewayDaemon::new(router);

            let requested: HashSet<String> = adapters
                .into_iter()
                .flat_map(|s| s.split(',').map(str::trim).map(String::from).collect::<Vec<_>>())
                .collect();

            for name in &requested {
                if let Some(adapter) = codex_gateway::adapter::adapter_from_name(name) {
                    daemon.register_adapter(name.clone(), adapter);
                } else {
                    eprintln!("Warning: unknown adapter `{name}` — skipping");
                }
            }

            println!(
                "Gateway daemon starting with {} adapter(s)...",
                daemon.adapters().len()
            );
            println!("Press Ctrl+C to stop.");
            daemon.run().await?;
        }
        GatewaySubcommand::ListAdapters => {
            println!("Supported channel adapters:");
            for name in codex_gateway::adapter::ALL_ADAPTER_NAMES {
                println!("  • {name}");
            }
        }
        GatewaySubcommand::Status => {
            println!("Gateway status: not running (stub)");
        }
    }
    Ok(())
}
