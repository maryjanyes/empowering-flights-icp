use std::{rc::Rc, str::FromStr};

use candid::candid_method;
use eth_rpc::call_contract;
use ethers_core::{
    abi::{Contract, Token},
    types::{Address, RecoveryMessage, Signature},
};
use util::to_hex;

mod eth_rpc;
mod util;

thread_local! {
    static EFMOCK: Rc<Contract> = Rc::new(include_abi!("../abi/efmockAbi.json"));
}

/// Verify if flight is on the running (Ethereum call).
#[ic_cdk_macros::update]
#[candid_method]
pub async fn verify_flight_is_alive(contract_address: String) -> bool {
    let abi = &EFMOCK.with(Rc::clone);
    let result = call_contract(
        "sepolia",
        contract_address,
        abi,
        "verify_flight_is_alive",
        &[],
    ).await;
    match result.get(0) {
        Some(Token::Bool(n)) => n.clone(),
        _ => panic!("Unexpected result"),
    }
}

/// Push flight coord (Ethereum call).
#[ic_cdk_macros::update]
#[candid_method]
pub async fn broadcast_flight_coords(contract_address: String, lat: String, long: String) {
    let abi = &EFMOCK.with(Rc::clone);
    let result = call_contract(
        "sepolia",
        contract_address,
        abi,
        "push_flight_coord",
        &[
            ethers_core::abi::Token::String(lat),
            ethers_core::abi::Token::String(long)
        ],
    ).await;
}

/// Owe flight (Ethereum call).
#[ic_cdk_macros::update]
#[candid_method]
pub async fn owe_flight(contract_address: String, max_owe_amount_to_next_user: u64) -> bool {
    let abi = &EFMOCK.with(Rc::clone);
    let result = call_contract(
        "sepolia",
        contract_address,
        abi,
        "owe_flight",
        &[
            ethers_core::abi::Token::Int(max_owe_amount_to_next_user.into())
        ],
    ).await;
    match result.get(0) {
        Some(Token::Bool(n)) => {
            return n.clone();
        },
        _ => panic!("Unexpected result"),
    }
}

/// Get all flights coords (Ethereum call).
#[ic_cdk_macros::update]
#[candid_method]
pub async fn get_flight_coords(contract_address: String) -> String {
    let abi = &EFMOCK.with(Rc::clone);
    let result = call_contract(
        "sepolia",
        contract_address,
        abi,
        "get_flight_coords",
        &[]
    ).await;
    match result.get(0) {
        Some(Token::Array(n)) => {
            let coords_vec: String = n.to_vec().iter().map(|x| <Token as Clone>::clone(&x).into_string().unwrap()).collect::<String>();
            format!("{:?}", coords_vec)
        },
        _ => panic!("Unexpected result"),
    }
}
