use unc_sdk::borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::json_types::Base64VecU8;
use unc_sdk::serde::{Deserialize, Serialize};
use unc_sdk::{ext_contract, require, UncSchema};

pub const FT_METADATA_SPEC: &str = "ft-1.0.0";

#[derive(Clone, BorshDeserialize, BorshSerialize, Deserialize, Serialize, UncSchema)]
#[serde(crate = "unc_sdk::serde")]
#[borsh(crate = "unc_sdk::borsh")]
pub struct FungibleTokenMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
    pub decimals: u8,
}

#[ext_contract(ext_ft_metadata)]
pub trait FungibleTokenMetadataProvider {
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}

impl FungibleTokenMetadata {
    pub fn assert_valid(&self) {
        require!(self.spec == FT_METADATA_SPEC);
        require!(self.reference.is_some() == self.reference_hash.is_some());
        if let Some(reference_hash) = &self.reference_hash {
            require!(reference_hash.0.len() == 32, "Hash has to be 32 bytes");
        }
    }
}
