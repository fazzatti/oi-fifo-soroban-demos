use soroban_sdk::{contracttype, Address, Vec};

pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 518400; // 30 days
pub(crate) const INSTANCE_BUMP_THREASHOLD: u32 = 120960; // 7 days

//TODO: Affilition between accounts validation
// #[derive(Clone)]
// #[contracttype]
// pub struct AffiliationDataKey {
//     pub primary: Address,
//     pub secondary: Address,
// }

// pub struct AffiliationValue {
//     pub inflow: i128,
//     pub outflow: i128,
// }

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
pub struct TxEntry {
    pub amount: i128,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct AccountQuotaReleaseData {
    pub outflow: Vec<TxReleaseEntry>,
    pub inflow: Vec<TxReleaseEntry>,
}

#[derive(Clone)]
#[contracttype]
pub struct TxReleaseEntry {
    pub amount: i128,
    pub time_left: u64,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    // Affiliation(AffiliationDataKey), //AffiliationValue
    Admin,                          //Address
    Asset,                          //Address
    AccountActivity(Address),       //AccountActivityData
    OutflowLimit,                   //i128
    InflowLimit,                    //i128
    QuotaTimeLimit,                 //u64
    ProbationPeriod,                //u64
    AccountProbationStart(Address), //u64
}
