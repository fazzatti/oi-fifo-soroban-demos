use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    // Affiliation(AffiliationDataKey), //AffiliationValue
    Admin,                          //Address
    Asset,                          //Address
    AssetController                //Address
}