use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use types::{AttestationDuty, Epoch, PublicKey, Slot};

/// When work needs to be performed by a validator, this type is given back to the main service
/// which indicates all the information that required to process the work.
///
/// Note: This is calculated per slot, so a validator knows which slot is related to this struct.
#[derive(Debug, Clone)]
pub struct WorkInfo {
    /// Validator needs to produce a block.
    pub produce_block: bool,
    /// Validator needs to produce an attestation. This supplies the required attestation data.
    pub attestation_duty: Option<AttestationDuty>,
}

/// The information required for a validator to propose and attest during some epoch.
///
/// Generally obtained from a Beacon Node, this information contains the validators canonical index
/// (their sequence in the global validator induction process) and the "shuffling" for that index
/// for some epoch.
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct EpochDuty {
    pub block_production_slot: Option<Slot>,
    pub attestation_slot: Slot,
    pub attestation_shard: u64,
    pub committee_index: u64,
}

impl EpochDuty {
    /// Returns `WorkInfo` if work needs to be done in the supplied `slot`
    pub fn is_work_slot(&self, slot: Slot) -> Option<WorkInfo> {
        // if validator is required to produce a slot return true
        let produce_block = match self.block_production_slot {
            Some(s) if s == slot => true,
            _ => false,
        };

        // if the validator is required to attest to a shard, create the data
        let mut attestation_duty = None;
        if self.attestation_slot == slot {
            attestation_duty = Some(AttestationDuty {
                slot,
                shard: self.attestation_shard,
                committee_index: self.committee_index as usize,
            });
        }

        if produce_block | attestation_duty.is_some() {
            return Some(WorkInfo {
                produce_block,
                attestation_duty,
            });
        }
        None
    }
}
/// Maps a list of public keys (many validators) to an EpochDuty.
pub type EpochDuties = HashMap<PublicKey, Option<EpochDuty>>;

pub enum EpochDutiesMapError {
    Poisoned,
    UnknownEpoch,
    UnknownValidator,
}

/// Maps an `epoch` to some `EpochDuties` for a single validator.
pub struct EpochDutiesMap {
    pub slots_per_epoch: u64,
    pub map: HashMap<Epoch, EpochDuties>,
}

impl EpochDutiesMap {
    pub fn new(slots_per_epoch: u64) -> Self {
        Self {
            slots_per_epoch,
            map: HashMap::new(),
        }
    }
}

// Expose the hashmap methods
impl Deref for EpochDutiesMap {
    type Target = HashMap<Epoch, EpochDuties>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
impl DerefMut for EpochDutiesMap {
    fn deref_mut(&mut self) -> &mut HashMap<Epoch, EpochDuties> {
        &mut self.map
    }
}

impl EpochDutiesMap {
    /// Checks if the validator has work to do.
    pub fn is_work_slot(
        &self,
        slot: Slot,
        pubkey: &PublicKey,
    ) -> Result<Option<WorkInfo>, EpochDutiesMapError> {
        let epoch = slot.epoch(self.slots_per_epoch);

        let epoch_duties = self
            .map
            .get(&epoch)
            .ok_or_else(|| EpochDutiesMapError::UnknownEpoch)?;
        if let Some(epoch_duty) = epoch_duties.get(pubkey) {
            if let Some(duty) = epoch_duty {
                // Retrieves the duty for a validator at a given slot
                return Ok(duty.is_work_slot(slot));
            } else {
                // the validator isn't active
                return Ok(None);
            }
        } else {
            // validator isn't known
            return Err(EpochDutiesMapError::UnknownValidator);
        }
    }
}

// TODO: add tests.
