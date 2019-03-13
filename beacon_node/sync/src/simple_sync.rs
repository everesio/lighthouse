use libp2p::PeerId;
use std::collections::HashMap;
use types::{Hash256, Slot};

/// Keeps track of syncing information for known connected peers.
pub struct PeerSyncInfo {
    best_slot: Slot,
    best_slot_hash: Hash256,
}

/// The current syncing state.
pub enum SyncState {
    Idle,
    Downloading,
    Stopped,
}

/// Simple Syncing protocol.
pub struct SimpleSync {
    genesis_hash: Hash256,
    known_peers: HashMap<PeerId, PeerSyncInfo>,
    state: SyncState,
}

impl SimpleSync {
    pub fn new(genesis_hash: Hash256) -> Self {
        SimpleSync {
            genesis_hash,
            known_peers: HashMap::new(),
            state: SyncState::Idle,
        }
    }
}