#![allow(dead_code)]

pub mod protocol;
pub mod render;

use protocol::A2uiProtocol;

/// Live Canvas (A2UI) engine for agent-driven interactive UI components.
///
/// The canvas allows agents to render widgets (forms, tables, charts,
/// buttons, etc.) into a visual workspace that the user can interact with.
#[derive(Debug, Default)]
pub struct CanvasEngine {
    protocol: A2uiProtocol,
}

impl CanvasEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Process an A2UI protocol message and produce render instructions.
    pub fn process(&mut self, msg: protocol::CanvasMessage) -> anyhow::Result<render::RenderPlan> {
        self.protocol.handle_message(msg)
    }
}
