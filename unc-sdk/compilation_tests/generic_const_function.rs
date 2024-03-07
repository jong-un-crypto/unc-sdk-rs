//! Functions can't use const generics.

use borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::unc_bindgen;

#[unc_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Ident {
    value: u32,
}

#[unc_bindgen]
impl Ident {
    pub fn is_ident_const<const N: usize>(&self, val: [u32; N]) -> [u32; N] {
        val
    }
}

fn main() {}
