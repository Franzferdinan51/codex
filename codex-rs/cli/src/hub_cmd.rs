use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;

/// Manage the DuckHive Hub (skill marketplace).
#[derive(Debug, Args)]
pub struct HubCommand {
    #[command(subcommand)]
    pub subcommand: HubSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum HubSubcommand {
    /// Search the hub for skills.
    Search {
        /// Search query string.
        #[arg(value_name = "QUERY")]
        query: String,
        /// Hub base URL (defaults to the public DuckHive Hub).
        #[arg(long, value_name = "URL", default_value = "https://hub.duckhive.ai")]
        hub_url: String,
    },
    /// Install a skill from the hub.
    Install {
        /// Skill ID to install.
        #[arg(value_name = "SKILL_ID")]
        skill_id: String,
        /// Destination directory (defaults to current dir).
        #[arg(value_name = "DEST", default_value = ".")]
        dest: PathBuf,
        /// Hub base URL.
        #[arg(long, value_name = "URL", default_value = "https://hub.duckhive.ai")]
        hub_url: String,
    },
    /// Show hub status.
    Status,
}

pub async fn run(cmd: HubCommand) -> Result<()> {
    match cmd.subcommand {
        HubSubcommand::Search { query, hub_url } => {
            let client = codex_hub::HubClient::new(hub_url);
            let results = client.search(&query).await?;
            if results.is_empty() {
                println!("No skills found for '{query}'.");
            } else {
                println!("Found {} skill(s):", results.len());
                for skill in results {
                    println!(
                        "  • {} by {} — {}",
                        skill.name, skill.author, skill.description
                    );
                }
            }
        }
        HubSubcommand::Install {
            skill_id,
            dest,
            hub_url,
        } => {
            let client = codex_hub::HubClient::new(hub_url);
            client.install(&skill_id, &dest).await?;
            println!("Skill {skill_id} installed to {}", dest.display());
        }
        HubSubcommand::Status => {
            println!("DuckHive Hub status: connected (stub)");
        }
    }
    Ok(())
}
