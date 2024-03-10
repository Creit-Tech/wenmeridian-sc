#![cfg(test)]

use crate::storage::core::CoreDataFunc;
use crate::storage::record::TLD;
use crate::tests::test_utils::{create_test_data, start_test_data, TestData};
use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
use soroban_sdk::{Address, Bytes, Env, IntoVal};

#[test]
fn test_writes() {
    let e: Env = Env::default();
    let test_data: TestData = create_test_data(&e);
    start_test_data(&e, &test_data);

    let participant: Address = Address::generate(&e);
    let participant_domain: Bytes = Bytes::from_slice(&e, "stellar".as_bytes());
    let tld: Bytes = Bytes::from_array(&e, &TLD);

    test_data
        .collateral_token_admin_client
        .mock_all_auths()
        .mint(&participant, &10000000000);

    test_data
        .registry_contract_client
        .mock_all_auths()
        .set_record(&participant_domain, &tld, &participant, &participant, &100);

    test_data
        .wen_meridian_contract_client
        .mock_auths(&[MockAuth {
            address: &participant,
            invoke: &MockAuthInvoke {
                contract: &test_data.wen_meridian_contract_client.address,
                fn_name: "write",
                args: (
                    participant_domain.clone(),
                    8i32,
                    975894616013605u64,
                    -79i32,
                    52536492536188u64,
                )
                    .into_val(&e),
                sub_invokes: &[],
            },
        }])
        .write(
            &participant_domain,
            &8i32,
            &975894616013605u64,
            &-79i32,
            &52536492536188u64,
        );

    e.as_contract(&test_data.wen_meridian_contract_client.address, || {
        assert_eq!(e._core_data().unwrap().last, 1);
    });
}
