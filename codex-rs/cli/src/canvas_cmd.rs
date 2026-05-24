use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage the DuckHive Live Canvas (A2UI) interactive workspace.
#[derive(Debug, Args)]
pub struct CanvasCommand {
    #[command(subcommand)]
    pub subcommand: CanvasSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CanvasSubcommand {
    /// Parse a JSON A2UI message and show the render plan.
    Render {
        /// JSON message to render (reads from stdin if omitted).
        #[arg(value_name = "JSON")]
        json: Option<String>,
    },
    /// List all supported canvas component types.
    Components,
}

pub async fn run(cmd: CanvasCommand) -> Result<()> {
    match cmd.subcommand {
        CanvasSubcommand::Render { json } => {
            let input = match json {
                Some(j) => j,
                None => {
                    let mut buf = String::new();
                    std::io::stdin().read_line(&mut buf)?;
                    buf
                }
            };
            let msg: codex_canvas::protocol::CanvasMessage = serde_json::from_str(&input)?;
            let mut engine = codex_canvas::CanvasEngine::new();
            let plan = engine.process(msg)?;
            let renderer = codex_canvas::render::TextRenderer::new();
            println!("{}", renderer.render(&plan));
        }
        CanvasSubcommand::Components => {
            println!("Supported canvas components:");
            println!("  • text");
            println!("  • button");
            println!("  • input");
            println!("  • table");
            println!("  • chart (bar, line, pie, scatter)");
            println!("  • form");
            println!("  • image");
            println!("  • code");
        }
    }
    Ok(())
}
