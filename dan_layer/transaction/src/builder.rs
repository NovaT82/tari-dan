//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use std::borrow::Borrow;

use tari_common_types::types::PrivateKey;
use tari_dan_common_types::{Epoch, SubstateAddress};
use tari_engine_types::{
    confidential::ConfidentialClaim,
    instruction::Instruction,
    substate::SubstateId,
    TemplateAddress,
};
use tari_template_lib::{
    args,
    args::Arg,
    models::{Amount, ComponentAddress, ConfidentialWithdrawProof, ResourceAddress},
};

use crate::{signature::TransactionSignatureFields, Transaction, TransactionSignature};

#[derive(Debug, Clone, Default)]
pub struct TransactionBuilder {
    instructions: Vec<Instruction>,
    fee_instructions: Vec<Instruction>,
    signature: Option<TransactionSignature>,
    inputs: Vec<SubstateAddress>,
    input_refs: Vec<SubstateAddress>,
    outputs: Vec<SubstateAddress>,
    min_epoch: Option<Epoch>,
    max_epoch: Option<Epoch>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            fee_instructions: Vec::new(),
            signature: None,
            inputs: Vec::new(),
            input_refs: Vec::new(),
            outputs: Vec::new(),
            min_epoch: None,
            max_epoch: None,
        }
    }

    /// Adds a fee instruction that calls the "take_fee" method on a component.
    /// This method must exist and return a Bucket with containing revealed confidential XTR resource.
    /// This allows the fee to originate from sources other than the transaction sender's account.
    /// The fee instruction will lock up the "max_fee" amount for the duration of the transaction.
    pub fn fee_transaction_pay_from_component(self, component_address: ComponentAddress, max_fee: Amount) -> Self {
        self.add_fee_instruction(Instruction::CallMethod {
            component_address,
            method: "pay_fee".to_string(),
            args: args![max_fee],
        })
    }

    /// Adds a fee instruction that calls the "take_fee_confidential" method on a component.
    /// This method must exist and return a Bucket with containing revealed confidential XTR resource.
    /// This allows the fee to originate from sources other than the transaction sender's account.
    pub fn fee_transaction_pay_from_component_confidential(
        self,
        component_address: ComponentAddress,
        proof: ConfidentialWithdrawProof,
    ) -> Self {
        self.add_fee_instruction(Instruction::CallMethod {
            component_address,
            method: "pay_fee_confidential".to_string(),
            args: args![proof],
        })
    }

    pub fn call_function(self, template_address: TemplateAddress, function: &str, args: Vec<Arg>) -> Self {
        self.add_instruction(Instruction::CallFunction {
            template_address,
            function: function.to_string(),
            args,
        })
    }

    pub fn call_method(self, component_address: ComponentAddress, method: &str, args: Vec<Arg>) -> Self {
        self.add_instruction(Instruction::CallMethod {
            component_address,
            method: method.to_string(),
            args,
        })
    }

    pub fn drop_all_proofs_in_workspace(self) -> Self {
        self.add_instruction(Instruction::DropAllProofsInWorkspace)
    }

    pub fn put_last_instruction_output_on_workspace<T: AsRef<[u8]>>(self, label: T) -> Self {
        self.add_instruction(Instruction::PutLastInstructionOutputOnWorkspace {
            key: label.as_ref().to_vec(),
        })
    }

    pub fn claim_burn(self, claim: ConfidentialClaim) -> Self {
        self.add_instruction(Instruction::ClaimBurn { claim: Box::new(claim) })
    }

    pub fn create_proof(self, account: ComponentAddress, resource_addr: ResourceAddress) -> Self {
        // We may want to make this a native instruction
        self.add_instruction(Instruction::CallMethod {
            component_address: account,
            method: "create_proof_for_resource".to_string(),
            args: args![resource_addr],
        })
    }

    pub fn with_fee_instructions(mut self, instructions: Vec<Instruction>) -> Self {
        self.fee_instructions = instructions;
        // Reset the signature as it is no longer valid
        self.signature = None;
        self
    }

    pub fn with_fee_instructions_builder<F: FnOnce(TransactionBuilder) -> TransactionBuilder>(mut self, f: F) -> Self {
        let builder = f(TransactionBuilder::new());
        self.fee_instructions = builder.instructions;
        // Reset the signature as it is no longer valid
        self.signature = None;
        self
    }

    pub fn add_fee_instruction(mut self, instruction: Instruction) -> Self {
        self.fee_instructions.push(instruction);
        // Reset the signature as it is no longer valid
        self.signature = None;
        self
    }

    pub fn add_instruction(mut self, instruction: Instruction) -> Self {
        self.instructions.push(instruction);
        // Reset the signature as it is no longer valid
        self.signature = None;
        self
    }

    pub fn with_instructions(mut self, instructions: Vec<Instruction>) -> Self {
        self.instructions.extend(instructions);
        // Reset the signature as it is no longer valid
        self.signature = None;
        self
    }

    pub fn with_signature(mut self, signature: TransactionSignature) -> Self {
        self.signature = Some(signature);
        self
    }

    pub fn sign(mut self, secret_key: &PrivateKey) -> Self {
        let signature_fields = TransactionSignatureFields {
            fee_instructions: self.fee_instructions.clone(),
            instructions: self.instructions.clone(),
            inputs: self.inputs.clone(),
            input_refs: self.input_refs.clone(),
            min_epoch: self.min_epoch,
            max_epoch: self.max_epoch,
        };

        self.signature = Some(TransactionSignature::sign(secret_key, signature_fields));
        self
    }

    /// Add an input to be consumed
    pub fn add_input(mut self, input_object: SubstateAddress) -> Self {
        self.inputs.push(input_object);
        // Reset the signature as it is no longer valid
        self.signature = None;
        self
    }

    pub fn with_substate_inputs<I: IntoIterator<Item = (B, u32)>, B: Borrow<SubstateId>>(self, inputs: I) -> Self {
        self.with_inputs(
            inputs
                .into_iter()
                .map(|(a, v)| SubstateAddress::from_address(a.borrow(), v)),
        )
    }

    pub fn with_inputs<I: IntoIterator<Item = SubstateAddress>>(mut self, inputs: I) -> Self {
        self.inputs.extend(inputs);
        // Reset the signature as it is no longer valid
        self.signature = None;
        self
    }

    /// Add an input to be used without mutation
    pub fn add_input_ref(mut self, input_object: SubstateAddress) -> Self {
        self.input_refs.push(input_object);
        // Reset the signature as it is no longer valid
        self.signature = None;
        self
    }

    pub fn with_substate_input_refs<I: IntoIterator<Item = (B, u32)>, B: Borrow<SubstateId>>(self, inputs: I) -> Self {
        self.with_input_refs(
            inputs
                .into_iter()
                .map(|(a, v)| SubstateAddress::from_address(a.borrow(), v)),
        )
    }

    pub fn with_input_refs<I: IntoIterator<Item = SubstateAddress>>(mut self, inputs: I) -> Self {
        self.input_refs.extend(inputs);
        // Reset the signature as it is no longer valid
        self.signature = None;
        self
    }

    pub fn add_output(mut self, output_object: SubstateAddress) -> Self {
        self.outputs.push(output_object);
        self
    }

    pub fn with_substate_outputs<I: IntoIterator<Item = (B, u32)>, B: Borrow<SubstateId>>(self, outputs: I) -> Self {
        self.with_outputs(
            outputs
                .into_iter()
                .map(|(a, v)| SubstateAddress::from_address(a.borrow(), v)),
        )
    }

    pub fn with_outputs<I: IntoIterator<Item = SubstateAddress>>(mut self, outputs: I) -> Self {
        self.outputs.extend(outputs);
        self
    }

    pub fn add_output_ref(mut self, output_object: SubstateAddress) -> Self {
        self.outputs.push(output_object);
        self
    }

    pub fn with_min_epoch(mut self, min_epoch: Option<Epoch>) -> Self {
        self.min_epoch = min_epoch;
        // Reset the signature as it is no longer valid
        self.signature = None;
        self
    }

    pub fn with_max_epoch(mut self, max_epoch: Option<Epoch>) -> Self {
        self.max_epoch = max_epoch;
        // Reset the signature as it is no longer valid
        self.signature = None;
        self
    }

    pub fn build_as_instructions(mut self) -> Vec<Instruction> {
        self.instructions.drain(..).collect()
    }

    pub fn build(mut self) -> Transaction {
        Transaction::new(
            self.fee_instructions.drain(..).collect(),
            self.instructions.drain(..).collect(),
            self.signature.take().expect("not signed"),
            self.inputs,
            self.input_refs,
            vec![],
            self.min_epoch,
            self.max_epoch,
        )
    }
}
