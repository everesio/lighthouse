extern crate bls_aggregates;
extern crate ssz;

mod aggregate_public_key;
mod keypair;
mod public_key;
mod secret_key;
mod serde_vistors;

#[cfg(not(debug_assertions))]
mod aggregate_signature;
#[cfg(not(debug_assertions))]
mod signature;
#[cfg(not(debug_assertions))]
pub use crate::aggregate_signature::AggregateSignature;
#[cfg(not(debug_assertions))]
pub use crate::signature::Signature;

#[cfg(debug_assertions)]
mod fake_aggregate_signature;
#[cfg(debug_assertions)]
mod fake_signature;
#[cfg(debug_assertions)]
pub use crate::fake_aggregate_signature::FakeAggregateSignature as AggregateSignature;
#[cfg(debug_assertions)]
pub use crate::fake_signature::FakeSignature as Signature;

pub use crate::aggregate_public_key::AggregatePublicKey;
pub use crate::keypair::Keypair;
pub use crate::public_key::PublicKey;
pub use crate::secret_key::SecretKey;

pub const BLS_AGG_SIG_BYTE_SIZE: usize = 96;

use hashing::hash;
use ssz::ssz_encode;

/// Returns the withdrawal credentials for a given public key.
pub fn get_withdrawal_credentials(pubkey: &PublicKey, prefix_byte: u8) -> Vec<u8> {
    let hashed = hash(&ssz_encode(pubkey));
    let mut prefixed = vec![prefix_byte];
    prefixed.extend_from_slice(&hashed[1..]);

    prefixed
}

pub fn bls_verify_aggregate(
    pubkey: &AggregatePublicKey,
    message: &[u8],
    signature: &AggregateSignature,
    domain: u64,
) -> bool {
    signature.verify(message, domain, pubkey)
}
