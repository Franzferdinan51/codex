use anyhow::Result;

use crate::message::InboundMessage;

/// Routes inbound messages to the appropriate agent / workspace based on
/// channel, sender, and content.
///
/// This is the OpenClaw-inspired multi-agent dispatch layer: different
/// channels or message patterns can be routed to dedicated agent
/// workspaces (e.g. "coding" agent vs "general" agent).
#[derive(Debug, Default)]
pub struct AgentRouter {
    // TODO: add routing rules, workspace registry, etc.
}

impl AgentRouter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Route a single inbound message. Returns after dispatching.
    pub async fn route(&self, msg: InboundMessage) -> Result<()> {
        tracing::info!(
            "routing message from {} via {}: {}",
            msg.sender.user_id,
            msg.channel.adapter,
            msg.content.text.chars().take(80).collect::<String>()
        );

        // TODO: implement actual routing logic:
        // 1. Match channel + sender against routing rules.
        // 2. Determine target workspace / agent.
        // 3. Forward the message into the app-server protocol.

        Ok(())
    }
}
