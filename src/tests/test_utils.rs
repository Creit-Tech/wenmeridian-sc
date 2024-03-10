#![cfg(test)]

use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{StellarAssetClient, TokenClient};
use soroban_sdk::{Address, Bytes, Env, String, Vec};

use crate::registry::{Client as RegistryClient, WASM};
use crate::storage::record::TLD;
use crate::{WenMeridianContract, WenMeridianContractClient};

fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (TokenClient<'a>, StellarAssetClient<'a>) {
    let contract_address = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_address),
        StellarAssetClient::new(e, &contract_address),
    )
}

pub struct TestData<'a> {
    pub collateral_token_admin: Address,
    pub collateral_token_client: TokenClient<'a>,
    pub collateral_token_admin_client: StellarAssetClient<'a>,

    pub registry_contract_client: RegistryClient<'a>,
    pub registry_contract_admin: Address,

    pub wen_meridian_contract_client: WenMeridianContractClient<'a>,
}

pub fn create_test_data(e: &Env) -> TestData {
    let collateral_token_admin: Address = Address::generate(&e);
    let (collateral_token_client, collateral_token_admin_client) =
        create_token_contract(&e, &collateral_token_admin);

    let registry: Address = e.register_contract_wasm(None, WASM);
    let registry_contract_client: RegistryClient = RegistryClient::new(&e, &registry);
    let registry_contract_admin: Address = Address::generate(&e);

    let contract_id: Address = e.register_contract(None, WenMeridianContract);
    let wen_meridian_contract_client: WenMeridianContractClient =
        WenMeridianContractClient::new(&e, &contract_id);

    TestData {
        collateral_token_admin,
        collateral_token_client,
        collateral_token_admin_client,
        registry_contract_client,
        registry_contract_admin,
        wen_meridian_contract_client,
    }
}

pub fn start_test_data(e: &Env, test_data: &TestData) {
    test_data.registry_contract_client.init(
        &test_data.registry_contract_admin,
        &300,
        &test_data.collateral_token_client.address,
        &0,
        &Vec::from_array(&e, [Bytes::from_array(&e, &TLD)]),
    );

    test_data.wen_meridian_contract_client.init(
        &String::from_str(&e, "Meridian 2024"),
        &100000,
        &test_data.registry_contract_client.address,
    );
}
