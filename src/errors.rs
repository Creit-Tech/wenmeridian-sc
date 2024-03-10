use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SCErrors {
    AlreadyStarted = 1,
    DeadlineReached = 2,
    InvalidDomain = 3,
    AlreadyExists = 4,
}
