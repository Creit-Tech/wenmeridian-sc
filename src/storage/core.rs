use soroban_sdk::{contracttype, Address, Env, String};

#[contracttype]
pub struct CoreData {
    // Name of the game (just in case we continue in next years)
    pub name: String,

    // Timestamp where this contract won't receive more participants
    pub deadline: u64,

    pub registry: Address,

    // The last is the latest index record
    pub last: u32,
}

#[contracttype]
pub enum CoreDataKeys {
    CoreData,
}

pub trait CoreDataFunc {
    fn _bump_instance(&self);
    fn _core_data(&self) -> Option<CoreData>;
    fn _set_core(&self, v: &CoreData);
}

impl CoreDataFunc for Env {
    fn _bump_instance(&self) {
        self.storage()
            .instance()
            .extend_ttl(17280 * 15, self.ledger().sequence() + (17280 * 30))
    }

    fn _core_data(&self) -> Option<CoreData> {
        self.storage().instance().get(&CoreDataKeys::CoreData)
    }

    fn _set_core(&self, v: &CoreData) {
        self.storage().instance().set(&CoreDataKeys::CoreData, v);
    }
}
