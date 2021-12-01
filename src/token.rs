use crate::*;
use near_sdk::json_types::U128;
use near_sdk::{env, log};

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn ft_mint(&mut self, amount: U128) {
        self.upgrade.assert_owner();

        let account_id = env::predecessor_account_id();
        assert!(self.ft.accounts.contains_key(&account_id), "ERR_ACCOUNT_NOT_REGISTERED");
        self.ft.internal_deposit(&account_id, amount.into());
        log!("Mint {} tokens to {}", u128::from(amount), account_id);
    }

    #[payable]
    pub fn ft_burn(&mut self, amount: U128) {
        self.upgrade.assert_owner();

        let account_id = env::predecessor_account_id();
        let amount = amount.into();
        self.ft.internal_withdraw(&account_id, amount);
        log!("Burn {} tokens from {}", amount, account_id);
    }
}
