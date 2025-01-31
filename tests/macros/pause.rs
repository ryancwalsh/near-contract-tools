use near_contract_tools::{
    pause::{Pause, PauseExternal},
    Pause,
};
use near_sdk::{
    borsh::{self, BorshSerialize},
    near_bindgen, BorshStorageKey,
};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Pause,
}

#[derive(Pause)]
#[near_bindgen]
struct ContractImplicitKey {}

#[derive(Pause)]
#[pause(storage_key = "StorageKey::Pause")]
#[near_bindgen]
struct Contract {
    pub value: u32,
}

#[near_bindgen]
impl Contract {
    pub fn only_when_unpaused(&mut self, value: u32) {
        Self::require_unpaused();

        self.value = value;
    }

    pub fn only_when_paused(&mut self, value: u32) {
        Self::require_paused();

        self.value = value;
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }
}

#[test]
fn derive_pause() {
    let mut contract = Contract { value: 0 };

    assert_eq!(
        contract.paus_is_paused(),
        false,
        "Initial state should be unpaused",
    );

    Contract::require_unpaused();

    contract.pause();

    assert_eq!(
        contract.paus_is_paused(),
        true,
        "Pausing the contract works",
    );

    Contract::require_paused();

    contract.unpause();

    assert_eq!(
        contract.paus_is_paused(),
        false,
        "Unpausing the contract works",
    );

    Contract::require_unpaused();
}

#[test]
fn derive_pause_methods() {
    let mut contract = Contract { value: 0 };

    contract.only_when_unpaused(5);

    assert_eq!(contract.get_value(), 5);

    contract.pause();

    contract.only_when_paused(10);

    assert_eq!(contract.get_value(), 10);
}

#[test]
#[should_panic(expected = "Disallowed while contract is unpaused")]
fn derive_pause_methods_fail_unpaused() {
    let mut contract = Contract { value: 0 };

    contract.only_when_paused(5);
}

#[test]
#[should_panic(expected = "Disallowed while contract is paused")]
fn derive_pause_methods_fail_paused() {
    let mut contract = Contract { value: 0 };

    contract.pause();

    contract.only_when_unpaused(5);
}
