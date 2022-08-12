// https://github.com/near/workspaces-rs/blob/8f12f3dc3b0251ac3f44ddf6ab6fc63003579139/workspaces/tests/create_account.rs

#![recursion_limit = "256"]

use near_contract_tools::FungibleToken;

use near_sdk::{json_types::U128, log, near_bindgen, ONE_YOCTO};
use near_units::parse_near;
use workspaces::{prelude::*, Account, Contract, DevNetwork, Worker};

#[derive(FungibleToken)]
#[fungible_token(
    name = "My Fungible Token",
    symbol = "MYFT",
    decimals = 18,
    icon = "https://example.com/icon.png",
    reference = "https://example.com/metadata.json",
    reference_hash = "YXNkZg==",
    no_hooks
)]
#[near_bindgen]
struct FungibleToken {}

// async fn register_user(
//     worker: &Worker<impl Network>,
//     contract: &Contract,
//     account_id: &AccountId,
// ) -> anyhow::Result<()> {
//     // https://github.com/near/near-sdk-rs/blob/a903f8c44a7be363d960838d92afdb22d1ce8b87/examples/fungible-token/tests/workspaces.rs#L8
//     let res = contract
//         .call(&worker, "storage_deposit")
//         .args_json((account_id, Option::<bool>::None))?
//         .gas(300_000_000_000_000)
//         .deposit(near_sdk::env::storage_byte_cost() * 125)
//         .transact()
//         .await?;
//     assert!(res.is_success());

//     Ok(())
// }

async fn init(
    worker: &Worker<impl DevNetwork>,
    //initial_balance: U128,
) -> anyhow::Result<(Contract, Account)> {
    // Inspired by https://github.com/near/near-sdk-rs/blob/a903f8c44a7be363d960838d92afdb22d1ce8b87/examples/fungible-token/tests/workspaces.rs#L25
    let ft_contract = worker
        .dev_deploy(
            &include_bytes!("../target/wasm32-unknown-unknown/release/near_contract_tools.wasm")
                .to_vec(),
        )
        .await?;

    log!("ft_contract {}", ft_contract.id());

    // let res = ft_contract
    //     .call(&worker, "new_default_meta")
    //     .args_json((ft_contract.id(), initial_balance))?
    //     .gas(300_000_000_000_000)
    //     .transact()
    //     .await?;
    // assert!(res.is_success());

    log!("Create alice...");

    let alice = ft_contract
        .as_account()
        .create_subaccount(&worker, "alice")
        .initial_balance(parse_near!("10 N")) // native NEAR
        .transact()
        .await?
        .into_result()?;

    log!("alice {}", alice.id());
    //register_user(worker, &ft_contract, alice.id()).await?;

    // let res = ft_contract
    //     .call(&worker, "storage_deposit")
    //     .args_json((alice.id(), Option::<bool>::None))?
    //     .gas(300_000_000_000_000)
    //     .deposit(near_sdk::env::storage_byte_cost() * 125)
    //     .transact()
    //     .await?;
    // assert!(res.is_success());

    return Ok((ft_contract, alice));
}

#[tokio::test]
async fn test_simple_transfer() -> anyhow::Result<()> {
    // Inspired by https://github.com/near/near-sdk-rs/blob/a903f8c44a7be363d960838d92afdb22d1ce8b87/examples/fungible-token/tests/workspaces.rs#L84
    let initial_balance = U128::from(parse_near!("1000 N"));
    let transfer_amount = U128::from(parse_near!("100 N"));
    let worker = workspaces::sandbox().await?;
    let (contract, alice) = init(
        &worker,
        //initial_balance
    )
    .await?;

    log!(
        "alice {} with native initial_balance {}",
        alice.id(),
        near_units::near::to_human(alice.view_account(&worker).await?.balance)
    );
    let res = contract
        .call(&worker, "ft_transfer")
        .args_json((alice.id(), transfer_amount, Option::<bool>::None))?
        .gas(300_000_000_000_000)
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
    assert!(res.is_success());

    let root_balance = contract
        .call(&worker, "ft_balance_of")
        .args_json((contract.id(),))?
        .view()
        .await?
        .json::<U128>()?;
    let alice_balance = contract
        .call(&worker, "ft_balance_of")
        .args_json((alice.id(),))?
        .view()
        .await?
        .json::<U128>()?;
    assert_eq!(initial_balance.0 - transfer_amount.0, root_balance.0);
    assert_eq!(transfer_amount.0, alice_balance.0);

    Ok(())
}

// TODO: Add tests for ft_transfer_call and ft_resolve_transfer.
