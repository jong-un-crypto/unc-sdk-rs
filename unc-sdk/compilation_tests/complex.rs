//! Complex smart contract.

use borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::unc_bindgen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(
    BorshDeserialize, BorshSerialize, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum TypeA {
    Var1,
    Var2,
}

#[derive(
    BorshDeserialize, BorshSerialize, Eq, PartialEq, Hash, PartialOrd, Serialize, Deserialize,
)]
pub enum TypeB {
    Var1,
    Var2,
}

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Storage {
    map: HashMap<TypeA, TypeB>,
}

#[unc_bindgen]
impl Storage {
    pub fn insert(&mut self, key: TypeA, value: TypeB) -> Option<TypeB> {
        self.map.insert(key, value)
    }
}

fn main() {}
