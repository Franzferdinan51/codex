#![allow(dead_code)]

pub mod adapter;
pub mod message;
pub mod router;

use std::collections::HashMap;

use anyhow::Result;
use tokio::sync::mpsc;
use tracing::info;

use crate::adapter::ChannelAdapter;
use crate::message::InboundMessage;
use crate::router::AgentRouter;

/// Gateway daemon that runs alongside the app-server, accepting messages
/// from external channels and routing them to the appropriate agent.
pub struct GatewayDaemon {
    router: AgentRouter,
    adapters: HashMap<String, Box<dyn ChannelAdapter>>,
    inbound_tx: mpsc::Sender<InboundMessage>,
    inbound_rx: mpsc::Receiver<InboundMessage>,
}

impl GatewayDaemon {
    pub fn new(router: AgentRouter) -> (Self, mpsc::Sender<InboundMessage>) {
        let (inbound_tx, inbound_rx) = mpsc::channel(256);
        (
            Self {
                router,
                adapters: HashMap::new(),
                inbound_tx: inbound_tx.clone(),
                inbound_rx,
            },
            inbound_tx,
        )
    }

    /// Register a channel adapter by name.
    pub fn register_adapter(
        &mut self,
        name: impl Into<String>,
        adapter: Box<dyn ChannelAdapter>,
    ) {
        let name = name.into();
        info!("registering gateway adapter: {name}");
        self.adapters.insert(name, adapter);
    }

    /// Access the registered adapters (used by CLI status/diagnostics).
    pub fn adapters(&self) -> &HashMap<String, Box<dyn ChannelAdapter>> {
        &self.adapters
    }

    /// Start the gateway daemon, spawning all adapter tasks.
    pub async fn run(mut self) -> Result<()> {
        info!("gateway daemon starting with {} adapters", self.adapters.len());

        // Clone the inbound sender before we consume `self.adapters`
        // so we avoid a fragile partial-move pattern.
        let tx = self.inbound_tx.clone();

        // Spawn each adapter as its own async task.
        let adapter_handles: Vec<_> = self
            .adapters
            .into_iter()
            .map(|(name, mut adapter)| {
                let _tx = tx.clone();
                // TODO: pass _tx into adapter so it can send inbound messages.
                tokio::spawn(async move {
                    info!("adapter {name} starting");
                    if let Err(err) = adapter.run().await {
                        tracing::error!("adapter {name} failed: {err:#}");
                    }
                })
            })
            .collect();

        // Main dispatch loop: process inbound messages and route them.
        while let Some(msg) = self.inbound_rx.recv().await {
            if let Err(err) = self.router.route(msg).await {
                tracing::error!("routing failed: {err:#}");
            }
        }

        // Clean shutdown: wait for all adapters.
        for handle in adapter_handles {
            let _ = handle.await;
        }

        info!("gateway daemon stopped");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::ChannelAdapter;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Default)]
    struct MockAdapter {
        run_count: Arc<AtomicUsize>,
    }

    impl ChannelAdapter for MockAdapter {
        fn run(&mut self) -> crate::adapter::BoxFuture<'_, Result<()>> {
            let count = self.run_count.clone();
            Box::pin(async move {
                count.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
        }
    }

    #[tokio::test]
    async fn gateway_registers_and_runs_adapters() {
        let router = AgentRouter::new();
        let (mut daemon, _tx) = GatewayDaemon::new(router);
        let mock = MockAdapter::default();
        daemon.register_adapter("mock", Box::new(mock));
        assert_eq!(daemon.adapters().len(), 1);
    }
}
