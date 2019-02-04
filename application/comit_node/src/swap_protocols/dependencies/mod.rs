use crate::{
    connection_pool::ConnectionPool, ledger_query_service::DefaultLedgerQueryServiceApiClient,
    seed::Seed,
};
use std::{sync::Arc, time::Duration};

mod client_impl;

/// Represents the things you have access to when starting execution of a
/// protocol
#[allow(missing_debug_implementations)]
pub struct ProtocolDependencies<T, S> {
    pub ledger_events: LedgerEventDependencies,
    pub metadata_store: Arc<T>,
    pub state_store: Arc<S>,
    pub connection_pool: Arc<ConnectionPool>,
    pub seed: Seed,
}

impl<T, S> Clone for ProtocolDependencies<T, S> {
    fn clone(&self) -> Self {
        ProtocolDependencies {
            ledger_events: self.ledger_events.clone(),
            metadata_store: Arc::clone(&self.metadata_store),
            state_store: Arc::clone(&self.state_store),
            connection_pool: Arc::clone(&self.connection_pool),
            seed: self.seed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LedgerEventDependencies {
    pub lqs_client: Arc<DefaultLedgerQueryServiceApiClient>,
    pub lqs_bitcoin_poll_interval: Duration,
    pub lqs_ethereum_poll_interval: Duration,
}