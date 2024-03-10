#![no_std]

mod errors;
mod storage;
mod tests;

use crate::errors::SCErrors;
use crate::storage::core::{CoreData, CoreDataFunc};
use crate::storage::record::{Record, RecordFunc, TLD};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Bytes, BytesN, Env, String};

mod registry {
    soroban_sdk::contractimport!(file = "registry.wasm");
}

pub trait WenMeridianTrait {
    fn init(e: Env, name: String, deadline: u64, registry: Address);

    fn write(e: Env, name: Bytes, lat_int: i32, lat_frc: u64, lng_int: i32, lng_frc: u64);
}

#[contract]
pub struct WenMeridianContract;

#[contractimpl]
impl WenMeridianTrait for WenMeridianContract {
    fn init(e: Env, name: String, deadline: u64, registry: Address) {
        e._bump_instance();
        if e._core_data().is_some() {
            panic_with_error!(&e, &SCErrors::AlreadyStarted);
        }

        e._set_core(&CoreData {
            name,
            deadline,
            registry,
            last: 0,
        });
    }

    fn write(e: Env, name: Bytes, lat_int: i32, lat_frc: u64, lng_int: i32, lng_frc: u64) {
        e._bump_instance();
        let mut core: CoreData = e._core_data().unwrap();
        core.last += 1;
        e._set_core(&core);

        if core.deadline < e.ledger().timestamp() {
            panic_with_error!(&e, &SCErrors::DeadlineReached);
        }

        let registry_client = registry::Client::new(&e, &core.registry);

        let node: BytesN<32> = registry_client.parse_domain(&name, &Bytes::from_array(&e, &TLD));

        let record: registry::Record = registry_client
            .record(&registry::RecordKeys::Record(node))
            .unwrap_or_else(|| panic_with_error!(&e, &SCErrors::InvalidDomain));

        let domain: registry::Domain = match record {
            registry::Record::Domain(v) => v,
            registry::Record::SubDomain(_) => panic_with_error!(&e, &SCErrors::InvalidDomain),
        };

        domain.owner.require_auth();

        if e._record_index(&domain.node).is_some() {
            panic_with_error!(&e, &SCErrors::AlreadyExists);
        }

        e._set_record(&Record {
            index: core.last.clone(),
            domain: name.clone(),
            node: domain.node.clone(),
            lat_int,
            lat_frc,
            lng_int,
            lng_frc,
        });

        e._bump_record(&domain.node, &core.last);
    }
}
