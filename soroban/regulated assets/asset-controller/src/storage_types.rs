use soroban_sdk::{contracttype, Address};

pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 34560; // 2 days
pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 518400; // 30 days

#[derive(Clone)]
#[contracttype]
pub struct AffiliationDataKey {
    pub primary: Affiliate,
    pub secondary: Affiliate,
}

pub struct AffiliationValue {
    pub inflow: i128,
    pub outflow: i128,
}


#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

pub struct UserActivity {
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