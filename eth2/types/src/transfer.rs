use super::Slot;
use crate::test_utils::TestRandom;
use bls::{PublicKey, Signature};
use rand::RngCore;
use serde_derive::{Deserialize, Serialize};
use ssz::TreeHash;
use ssz_derive::{Decode, Encode, SignedRoot, TreeHash};
use test_random_derive::TestRandom;

/// The data submitted to the deposit contract.
///
/// Spec v0.5.0
#[derive(
    Debug,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
    TestRandom,
    SignedRoot,
)]
pub struct Transfer {
    pub sender: u64,
    pub recipient: u64,
    pub amount: u64,
    pub fee: u64,
    pub slot: Slot,
    pub pubkey: PublicKey,
    pub signature: Signature,
}

#[cfg(test)]
mod tests {
    use super::*;

    ssz_tests!(Transfer);
}
