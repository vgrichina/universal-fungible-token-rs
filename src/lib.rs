/*!
* NEP-141 Token contract
*
* The aim of the contract is to provide basic token contract where ownere controls minting and burning.
* It supports methods `ft_mint` and `ft_burn` that mint and burn underlying tokens.
*
* lib.rs is the main entry point.
* token.rs contains interfaces for minting and burning.
*/
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::upgrade::{Ownable, Upgradable, Upgrade};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128, U64};
// Needed by `impl_fungible_token_core` for old Rust.
#[allow(unused_imports)]
use near_sdk::env;
use near_sdk::{near_bindgen, AccountId, Duration, PanicOnDefault, Promise, PromiseOrValue, Timestamp};

mod token;

near_sdk::setup_alloc!();

/// Upgrade duration is 1 day.
const UPGRADE_STAGING_DURATION: Duration = 24 * 60 * 60 * 1_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub ft: FungibleToken,
    pub upgrade: Upgrade
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self {
            ft: FungibleToken::new(b"a".to_vec()),
            upgrade: Upgrade::new(owner.clone(), UPGRADE_STAGING_DURATION),
        }
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, ft);
near_contract_standards::impl_fungible_token_storage!(Contract, ft);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: String::from("Universal fungible token"),
            symbol: String::from("SHITCOIN"),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 18,
        }
    }
}

#[near_bindgen]
impl Ownable for Contract {
    fn get_owner(&self) -> AccountId {
        self.upgrade.get_owner()
    }

    fn set_owner(&mut self, owner: AccountId) {
        self.upgrade.set_owner(owner)
    }
}

#[near_bindgen]
impl Upgradable for Contract {
    fn get_staging_duration(&self) -> U64 {
        self.upgrade.get_staging_duration()
    }

    fn stage_code(&mut self, code: Vec<u8>, timestamp: Timestamp) {
        self.upgrade.stage_code(code, timestamp)
    }

    fn deploy_code(&mut self) -> Promise {
        self.upgrade.deploy_code()
    }

    fn migrate(&mut self) {
        self.upgrade.migrate()
    }
}