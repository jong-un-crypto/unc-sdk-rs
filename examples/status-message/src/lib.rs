use unc_sdk::borsh::{BorshDeserialize, BorshSerialize};
use unc_sdk::store::LookupMap;
use unc_sdk::{env, log, unc_bindgen, AccountId, BorshStorageKey};

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "unc_sdk::borsh")]
struct RecordsKey;

#[unc_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "unc_sdk::borsh")]
pub struct StatusMessage {
    records: LookupMap<AccountId, String>,
}

impl Default for StatusMessage {
    fn default() -> Self {
        Self { records: LookupMap::new(RecordsKey) }
    }
}

#[unc_bindgen]
impl StatusMessage {
    #[payable]
    pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();
        log!("{} set_status with message {}", account_id, message);
        self.records.insert(account_id, message);
    }

    pub fn get_status(&self, account_id: AccountId) -> Option<&String> {
        log!("get_status for account_id {}", account_id);
        self.records.get(&account_id)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use unc_sdk::test_utils::{get_logs, VMContextBuilder};
    use unc_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_unc".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn set_get_message() {
        let context = get_context(false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        // Flush the pending changes to avoid panic in the view method below due to the pending non-commited changes to the `store::LookupMap`:
        // HostError(ProhibitedInView { method_name: "storage_write" })
        contract.records.flush();
        assert_eq!(get_logs(), vec!["bob_unc set_status with message hello"]);

        let context = get_context(true);
        testing_env!(context);
        assert_eq!("hello", contract.get_status("bob_unc".parse().unwrap()).unwrap());
        assert_eq!(get_logs(), vec!["get_status for account_id bob_unc"]);
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status("francis.unc".parse().unwrap()));
        assert_eq!(get_logs(), vec!["get_status for account_id francis.unc"])
    }
}
