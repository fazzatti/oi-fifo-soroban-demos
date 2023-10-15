use soroban_sdk::contracttype;

pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 518400; // 30 days
pub(crate) const INSTANCE_BUMP_THREASHOLD: u32 = 120960; // 7 days



#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    // Affiliation(AffiliationDataKey), //AffiliationValue
    Admin,                          //Address
    Asset,                          //Address
    AssetController                //Address
}