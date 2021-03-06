use super::DepositInput;
use crate::test_utils::TestRandom;
use rand::RngCore;
use serde_derive::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode, TreeHash};
use test_random_derive::TestRandom;

/// Data generated by the deposit contract.
///
/// Spec v0.5.0
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Encode, Decode, TreeHash, TestRandom)]
pub struct DepositData {
    pub amount: u64,
    pub timestamp: u64,
    pub deposit_input: DepositInput,
}

#[cfg(test)]
mod tests {
    use super::*;

    ssz_tests!(DepositData);
}
