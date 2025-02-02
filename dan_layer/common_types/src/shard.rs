//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use std::{fmt::Display, ops::RangeInclusive};

use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

use crate::{
    uint::{U256, U256_ONE},
    SubstateAddress,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "../../bindings/src/types/"))]
pub struct Shard(u32);

impl Shard {
    pub fn as_u32(self) -> u32 {
        self.0
    }

    pub fn to_substate_address_range(self, num_committees: u32) -> RangeInclusive<SubstateAddress> {
        if num_committees == 0 {
            return RangeInclusive::new(SubstateAddress::zero(), SubstateAddress::from_u256(U256::MAX));
        }
        let bucket = U256::from(self.0);
        let num_committees = U256::from(num_committees);
        let bucket_size = U256::MAX / num_committees;
        let bucket_remainder = U256::MAX % num_committees;
        let next_bucket = bucket + U256_ONE;
        let start = bucket_size * bucket + bucket_remainder.min(bucket);
        let mut end = start + bucket_size;
        if next_bucket != num_committees && bucket_remainder <= bucket {
            end -= U256_ONE;
        }
        RangeInclusive::new(SubstateAddress::from_u256(start), SubstateAddress::from_u256(end))
    }
}

impl From<u32> for Shard {
    fn from(v: u32) -> Self {
        Self(v)
    }
}

impl PartialEq<u32> for Shard {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}
impl PartialEq<Shard> for u32 {
    fn eq(&self, other: &Shard) -> bool {
        *self == other.as_u32()
    }
}

impl Display for Shard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u32())
    }
}

#[cfg(test)]
mod test {
    use crate::uint::{U256, U256_ONE};

    #[test]
    fn committee_is_properly_computed() {
        for num_of_committees in 1..100 {
            let mut previous_end = U256::ZERO;
            let mut min_committee_size = U256::MAX;
            let mut max_committee_size = U256::ZERO;
            for bucket_index in 0..num_of_committees {
                let bucket = super::Shard::from(bucket_index);
                let range = bucket.to_substate_address_range(num_of_committees);
                if bucket_index > 0 {
                    assert_eq!(
                        range.start().to_u256(),
                        previous_end + U256_ONE,
                        "Bucket should start where the previous one ended+1"
                    );
                }
                min_committee_size = min_committee_size.min(range.end().to_u256() - range.start().to_u256());
                max_committee_size = max_committee_size.max(range.end().to_u256() - range.start().to_u256());
                previous_end = range.end().to_u256();
            }
            assert!(
                num_of_committees <= 1 || max_committee_size <= min_committee_size + U256_ONE,
                "Committee sizes should be balanced {min_committee_size} {max_committee_size}"
            );
            assert_eq!(previous_end, U256::MAX, "Last bucket should end at U256::MAX");
        }
    }
}
