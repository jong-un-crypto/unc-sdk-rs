//! Smart contract with initialization function.

use borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::{unc_bindgen, PanicOnDefault};

#[unc_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
struct Incrementer {
    value: u32,
}

#[unc_bindgen]
impl Incrementer {
    pub fn inc(&mut self, by: u32) {
        self.value += by;
    }
    #[init]
    pub fn new(starting_value: u32) -> Self {
        Self { value: starting_value }
    }
}

fn main() {}
