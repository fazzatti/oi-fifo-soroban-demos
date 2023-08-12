use soroban_sdk::{contracttype, Address, Vec};

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
pub struct AccountActivityData {
    pub outflow: Vec<TxEntry>,
    pub inflow: Vec<TxEntry>,
}

#[derive(Clone)]
#[contracttype]
pub struct TxEntry{
    pub amount: i128,
    pub timestamp:u64,
}


#[derive(Clone)]
#[contracttype]
pub struct AccountQuotaReleaseData {
    pub outflow: Vec<TxReleaseEntry>,
    pub inflow: Vec<TxReleaseEntry>,
}

#[derive(Clone)]
#[contracttype]
pub struct TxReleaseEntry{
    pub amount: i128,
    pub time_left:u64,
}

// pub struct UserActivityData {
//     pub recent_outflow: i128,
//     pub recent_inflow: i128,
// }


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Affiliation(AffiliationDataKey),
    Admin,         
    Asset,                
    AccountActivity(Address),
    OutflowLimit,
    InflowLimit,
    QuotaTimeLimit,                  //u64
}
