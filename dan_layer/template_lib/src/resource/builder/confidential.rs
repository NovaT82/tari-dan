//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use super::TOKEN_SYMBOL;
use crate::{
    args::MintArg,
    auth::{AccessRule, OwnerRule, ResourceAccessRules},
    models::{Bucket, Metadata, ResourceAddress},
    prelude::ConfidentialOutputProof,
    resource::{ResourceManager, ResourceType},
};

/// Utility for building confidential resources inside templates
pub struct ConfidentialResourceBuilder {
    initial_supply_proof: Option<ConfidentialOutputProof>,
    metadata: Metadata,
    access_rules: ResourceAccessRules,
    owner_rule: OwnerRule,
}

impl ConfidentialResourceBuilder {
    /// Returns a new confidential resource builder
    pub(super) fn new() -> Self {
        Self {
            initial_supply_proof: None,
            metadata: Metadata::new(),
            access_rules: ResourceAccessRules::new(),
            owner_rule: OwnerRule::default(),
        }
    }

    /// Sets up who will be the owner of the resource.
    /// Resource owners are the only ones allowed to update the resource's access rules after creation
    pub fn with_owner_rule(mut self, rule: OwnerRule) -> Self {
        self.owner_rule = rule;
        self
    }

    /// Sets up who can access the resource for each type of action
    pub fn with_access_rules(mut self, rules: ResourceAccessRules) -> Self {
        self.access_rules = rules;
        self
    }

    /// Sets up who can mint new tokens of the resource
    pub fn mintable(mut self, rule: AccessRule) -> Self {
        self.access_rules = self.access_rules.mintable(rule);
        self
    }

    /// Sets up who can burn (destroy) tokens of the resource
    pub fn burnable(mut self, rule: AccessRule) -> Self {
        self.access_rules = self.access_rules.burnable(rule);
        self
    }

    /// Sets up who can recall tokens of the resource.
    /// A recall is the forceful withdrawal of tokens from any external vault
    pub fn recallable(mut self, rule: AccessRule) -> Self {
        self.access_rules = self.access_rules.recallable(rule);
        self
    }

    /// Sets up who can withdraw tokens of the resource from any vault
    pub fn withdrawable(mut self, rule: AccessRule) -> Self {
        self.access_rules = self.access_rules.withdrawable(rule);
        self
    }

    /// Sets up who can deposit tokens of the resource into any vault
    pub fn depositable(mut self, rule: AccessRule) -> Self {
        self.access_rules = self.access_rules.depositable(rule);
        self
    }

    /// Sets up the specified `symbol` as the token symbol in the metadata of the resource
    pub fn with_token_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.metadata.insert(TOKEN_SYMBOL, symbol);
        self
    }

    /// Adds a new metadata entry to the resource
    pub fn add_metadata<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Sets up all the metadata entries of the resource
    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Sets up how many tokens are going to be minted on resource creation
    pub fn initial_supply(mut self, initial_supply: ConfidentialOutputProof) -> Self {
        self.initial_supply_proof = Some(initial_supply);
        self
    }

    /// Build the resource, returning the address
    pub fn build(self) -> ResourceAddress {
        // TODO: Improve API
        assert!(
            self.initial_supply_proof.is_none(),
            "call build_bucket when initial supply is set"
        );
        let (address, _) = Self::build_internal(self.owner_rule, self.access_rules, self.metadata, None);
        address
    }

    /// Build the resource and return a bucket with the initial minted tokens (if specified previously)
    pub fn build_bucket(self) -> Bucket {
        let resource = MintArg::Confidential {
            proof: Box::new(
                self.initial_supply_proof
                    .expect("[build_bucket] initial supply not set"),
            ),
        };

        let (_, bucket) = Self::build_internal(self.owner_rule, self.access_rules, self.metadata, Some(resource));
        bucket.expect("[build_bucket] Bucket not returned from system")
    }

    fn build_internal(
        owner_rule: OwnerRule,
        access_rules: ResourceAccessRules,
        metadata: Metadata,
        resource: Option<MintArg>,
    ) -> (ResourceAddress, Option<Bucket>) {
        ResourceManager::new().create(ResourceType::Confidential, owner_rule, access_rules, metadata, resource)
    }
}
