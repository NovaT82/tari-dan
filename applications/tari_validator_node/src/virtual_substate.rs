//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use log::*;
use tari_common_types::types::PublicKey;
use tari_dan_common_types::Epoch;
use tari_dan_engine::runtime::VirtualSubstates;
use tari_dan_storage::{consensus_models::Block, StateStore, StorageError};
use tari_engine_types::{
    fee_claim::FeeClaim,
    virtual_substate::{VirtualSubstate, VirtualSubstateId},
};
use tari_epoch_manager::EpochManagerReader;
use tari_template_lib::models::Amount;

const LOG_TARGET: &str = "tari::dan::validator_node::virtual_substate";

#[derive(Debug, Clone)]
pub struct VirtualSubstateManager<TStateStore, TEpochManager> {
    epoch_manager: TEpochManager,
    store: TStateStore,
}

impl<TStateStore, TEpochManager> VirtualSubstateManager<TStateStore, TEpochManager>
where
    TStateStore: StateStore,
    TEpochManager: EpochManagerReader<Addr = TStateStore::Addr>,
{
    pub fn new(state_store: TStateStore, epoch_manager: TEpochManager) -> Self {
        Self {
            epoch_manager,
            store: state_store,
        }
    }

    pub async fn generate_for_address(
        &self,
        address: &VirtualSubstateId,
    ) -> Result<VirtualSubstate, VirtualSubstateError> {
        match address {
            VirtualSubstateId::CurrentEpoch => self.generate_current_epoch().await,
            VirtualSubstateId::UnclaimedValidatorFee { epoch, address } => {
                self.generate_validator_fee_claim(Epoch(*epoch), address)
            },
        }
    }

    pub fn get_virtual_substates(
        &self,
        claim_instructions: Vec<(Epoch, PublicKey)>,
    ) -> Result<VirtualSubstates, VirtualSubstateError> {
        let mut virtual_substates = VirtualSubstates::with_capacity(claim_instructions.len());

        self.store.with_read_tx(|tx| {
            for (epoch, validator_public_key) in claim_instructions {
                let fee_claim = self.generate_validator_fee_claim_inner(tx, epoch, &validator_public_key)?;

                info!(target: LOG_TARGET, "Adding permitted fee claim for epoch {}, {} with amount {}", epoch, validator_public_key, fee_claim.amount);
                virtual_substates.insert(
                    VirtualSubstateId::UnclaimedValidatorFee{epoch: epoch.as_u64(), address: validator_public_key},
                    VirtualSubstate::UnclaimedValidatorFee(fee_claim)
                );
            }

            Ok(virtual_substates)
        })
    }

    async fn generate_current_epoch(&self) -> Result<VirtualSubstate, VirtualSubstateError> {
        let current_epoch = self.epoch_manager.current_epoch().await?;
        Ok(VirtualSubstate::CurrentEpoch(current_epoch.as_u64()))
    }

    fn generate_validator_fee_claim(
        &self,
        epoch: Epoch,
        public_key: &PublicKey,
    ) -> Result<VirtualSubstate, VirtualSubstateError> {
        let claim = self
            .store
            .with_read_tx(|tx| self.generate_validator_fee_claim_inner(tx, epoch, public_key))?;
        Ok(VirtualSubstate::UnclaimedValidatorFee(claim))
    }

    fn generate_validator_fee_claim_inner(
        &self,
        tx: &mut <TStateStore as StateStore>::ReadTransaction<'_>,
        epoch: Epoch,
        public_key: &PublicKey,
    ) -> Result<FeeClaim, VirtualSubstateError> {
        let validator_fee = Block::get_total_due_for_epoch(tx, epoch, public_key)?;
        // If validator_fee == 0:
        // A validator may claim without knowing that they have no fees for the epoch.
        // As long as they can pay the fee for the transaction, we can add the 0 claim.

        Ok(FeeClaim {
            epoch: epoch.as_u64(),
            validator_public_key: public_key.clone(),
            amount: Amount::try_from(validator_fee).expect("Fee greater than Amount::MAX"),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VirtualSubstateError {
    #[error("Epoch manager error: {0}")]
    EpochManagerError(#[from] tari_epoch_manager::EpochManagerError),
    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),
}
