use soroban_sdk::{contracttype, Bytes, BytesN, Env};

pub const TLD: [u8; 3] = [120, 108, 109];

#[contracttype]
pub struct Record {
    pub index: u32,

    pub domain: Bytes,
    pub node: BytesN<32>,

    pub lat_int: i32,
    pub lat_frc: u64,
    pub lng_int: i32,
    pub lng_frc: u64,
}

#[contracttype]
pub enum RecordKeys {
    // By passing the node we get the record
    Record(u32),

    // By passing the index we can know the node
    RecordIndex(BytesN<32>),
}

pub trait RecordFunc {
    fn _bump_record(&self, node: &BytesN<32>, index: &u32);
    fn _record(&self, index: &u32) -> Option<Record>;
    fn _record_index(&self, node: &BytesN<32>) -> Option<u32>;
    fn _set_record(&self, v: &Record);
}

impl RecordFunc for Env {
    fn _bump_record(&self, node: &BytesN<32>, index: &u32) {
        self.storage().persistent().extend_ttl(
            &RecordKeys::RecordIndex(node.clone()),
            17280 * 15,
            self.ledger().sequence() + (17280 * 30),
        );
        self.storage().persistent().extend_ttl(
            &RecordKeys::Record(index.clone()),
            17280 * 15,
            self.ledger().sequence() + (17280 * 30),
        );
    }

    fn _record(&self, index: &u32) -> Option<Record> {
        self.storage()
            .persistent()
            .get(&RecordKeys::Record(index.clone()))
    }

    fn _record_index(&self, node: &BytesN<32>) -> Option<u32> {
        self.storage()
            .persistent()
            .get(&RecordKeys::RecordIndex(node.clone()))
    }

    fn _set_record(&self, v: &Record) {
        self.storage()
            .persistent()
            .set(&RecordKeys::Record(v.index.clone()), v);

        self.storage()
            .persistent()
            .set(&RecordKeys::RecordIndex(v.node.clone()), &v.index)
    }
}
