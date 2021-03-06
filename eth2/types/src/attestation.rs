use super::{AggregateSignature, AttestationData, Bitfield};
use crate::test_utils::TestRandom;
use rand::RngCore;
use serde_derive::{Deserialize, Serialize};
use ssz::TreeHash;
use ssz_derive::{Decode, Encode, SignedRoot, TreeHash};
use test_random_derive::TestRandom;

/// Details an attestation that can be slashable.
///
/// Spec v0.5.0
#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
    TestRandom,
    SignedRoot,
)]
pub struct Attestation {
    pub aggregation_bitfield: Bitfield,
    pub data: AttestationData,
    pub custody_bitfield: Bitfield,
    pub aggregate_signature: AggregateSignature,
}

#[cfg(test)]
mod tests {
    use super::*;

    ssz_tests!(Attestation);
}
