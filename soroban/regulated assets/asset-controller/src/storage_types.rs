use soroban_sdk::{contracttype, Address};

pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 34560; // 2 days
pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 518400; // 30 days

#[derive(Clone)]
#[contracttype]
pub struct AffiliationDataKey {
    pub primary: Address,
    pub secondary: Address,
}

pub struct AffiliationValue {
    pub inflow: i128,
    pub outflow: i128,
}

#[derive(Clone)]
#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct UserActivityData {
    pub recent_outflow: i128,
    pub recent_inflow: i128,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Affiliation(AffiliationDataKey),
    Admin,         
    Asset,                
    UserActivity(Address),
    OutflowLimit,
}