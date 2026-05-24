use anyhow::Result;
use clap::{Args, Subcommand};
use codex_utils_absolute_path::AbsolutePathBuf;

/// Manage DuckHive workspace SOUL.md and TOOLS.md configurations.
#[derive(Debug, Args)]
pub struct SoulCommand {
    #[command(subcommand)]
    pub subcommand: SoulSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum SoulSubcommand {
    /// Discover and display workspace configs from the current directory.
    Discover,
    /// Show the contents of a SOUL.md file.
    Show {
        /// Path to the workspace directory (defaults to current dir).
        #[arg(value_name = "DIR")]
        dir: Option<AbsolutePathBuf>,
    },
}

pub async fn run(cmd: SoulCommand) -> Result<()> {
    match cmd.subcommand {
        SoulSubcommand::Discover => {
            let cwd = AbsolutePathBuf::relative_to_current_dir(".")?;
            let configs = codex_soul::discover_workspace_configs(&cwd).await?;
            if configs.is_empty() {
                println!("No .duckhive/ workspace configs found in current path.");
            } else {
                println!("Discovered {} workspace config(s):", configs.len());
                for (path, config) in configs {
                    println!("\n  📁 {}", path.display());
                    if let Some(soul) = &config.soul {
                        if let Some(name) = &soul.name {
                            println!("     Name: {name}");
                        }
                        if let Some(desc) = &soul.description {
                            println!("     Description: {desc}");
                        }
                    }
                    if let Some(tools) = &config.tools {
                        println!("     Tools: {}", tools.tools.len());
                    }
                }
            }
        }
        SoulSubcommand::Show { dir } => {
            let dir = dir.unwrap_or_else(|| AbsolutePathBuf::relative_to_current_dir(".").unwrap());
            let config = codex_soul::WorkspaceConfig::load_from_dir(&dir).await?;
            if config.is_empty() {
                println!("No SOUL.md or TOOLS.md found in {}", dir.display());
            } else {
                if let Some(soul) = &config.soul {
                    println!("SOUL.md:");
                    println!("{soul:#?}");
                }
                if let Some(tools) = &config.tools {
                    println!("TOOLS.md:");
                    println!("{tools:#?}");
                }
            }
        }
    }
    Ok(())
}
