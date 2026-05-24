use std::future::Future;
use std::pin::Pin;

use anyhow::Result;

/// Object-safe future type returned by [`ChannelAdapter::run`].
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Trait for channel adapters that bridge external messaging platforms
/// (Slack, Discord, Telegram, WhatsApp, IRC, Matrix, etc.) into the
/// DuckHive gateway.
pub trait ChannelAdapter: Send {
    /// Start the adapter. This should block until the adapter shuts down.
    fn run(&mut self) -> BoxFuture<'_, Result<()>>;
}

// ---------------------------------------------------------------------------
// Stub implementations for each supported channel.
// Each stub logs that it would connect and then returns immediately.
// In a real deployment these would hold WebSocket / HTTP client state
// and forward messages into the gateway's inbound channel.
// ---------------------------------------------------------------------------

macro_rules! define_stub_adapter {
    ($name:ident, $display:expr) => {
        #[derive(Debug, Default)]
        pub struct $name;

        impl ChannelAdapter for $name {
            fn run(&mut self) -> BoxFuture<'_, Result<()>> {
                Box::pin(async move {
                    tracing::info!(concat!($display, " adapter started (stub)"));
                    // In a real implementation this would:
                    // 1. Connect to the platform's API / websocket.
                    // 2. Listen for incoming messages.
                    // 3. Parse them into `InboundMessage`.
                    // 4. Send them into the gateway's inbound channel.
                    //
                    // For now we park the task so it stays alive without
                    // consuming CPU.
                    std::future::pending::<()>().await;
                    Ok(())
                })
            }
        }
    };
}

define_stub_adapter!(SlackAdapter, "Slack");
define_stub_adapter!(DiscordAdapter, "Discord");
define_stub_adapter!(TelegramAdapter, "Telegram");
define_stub_adapter!(WhatsAppAdapter, "WhatsApp");
define_stub_adapter!(IrcAdapter, "IRC");
define_stub_adapter!(MatrixAdapter, "Matrix");
define_stub_adapter!(SignalAdapter, "Signal");
define_stub_adapter!(ImessageAdapter, "iMessage");

/// Create a named adapter from its identifier string.
pub fn adapter_from_name(name: &str) -> Option<Box<dyn ChannelAdapter>> {
    match name.to_ascii_lowercase().as_str() {
        "slack" => Some(Box::new(SlackAdapter)),
        "discord" => Some(Box::new(DiscordAdapter)),
        "telegram" => Some(Box::new(TelegramAdapter)),
        "whatsapp" => Some(Box::new(WhatsAppAdapter)),
        "irc" => Some(Box::new(IrcAdapter)),
        "matrix" => Some(Box::new(MatrixAdapter)),
        "signal" => Some(Box::new(SignalAdapter)),
        "imessage" => Some(Box::new(ImessageAdapter)),
        _ => None,
    }
}

/// List of all supported adapter names.
pub const ALL_ADAPTER_NAMES: &[&str] = &[
    "slack",
    "discord",
    "telegram",
    "whatsapp",
    "irc",
    "matrix",
    "signal",
    "imessage",
];
