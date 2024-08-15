use soroban_sdk::{contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
pub struct WrapperMetadata {
    pub enforced: bool,
    pub is_active: bool,
    pub admin: Address,
    pub asset: Address,
    pub asset_controller: Address,
}

#[derive(Clone)]
#[contracttype]
pub enum Metadatakey {
    WrapperMetadada,
}

pub fn write_metadata(e: &Env, metadata: &WrapperMetadata) {
    e.storage()
        .instance()
        .set(&Metadatakey::WrapperMetadada, metadata);
}

pub fn read_metadata(e: &Env) -> WrapperMetadata {
    e.storage()
        .instance()
        .get(&Metadatakey::WrapperMetadada)
        .unwrap()
}

pub fn write_admin(e: &Env, new_admin: Address) {
    let mut metadata = read_metadata(e);
    metadata.admin = new_admin;
    write_metadata(e, &metadata);
}

pub fn write_asset(e: &Env, new_asset: Address) {
    let mut metadata = read_metadata(e);
    metadata.asset = new_asset;
    write_metadata(e, &metadata);
}

pub fn write_asset_controller(e: &Env, new_asset_controller: Address) {
    let mut metadata = read_metadata(e);
    metadata.asset_controller = new_asset_controller;
    write_metadata(e, &metadata);
}

pub fn write_is_active(e: &Env, is_active: bool) {
    let mut metadata = read_metadata(e);
    metadata.is_active = is_active;
    write_metadata(e, &metadata);
}

pub fn write_enforced(e: &Env, enforced: bool) {
    let mut metadata = read_metadata(e);
    metadata.enforced = enforced;
    write_metadata(e, &metadata);
}

pub fn read_admin(e: &Env) -> Address {
    read_metadata(e).admin
}

pub fn read_asset(e: &Env) -> Address {
    read_metadata(e).asset
}

pub fn read_asset_controller(e: &Env) -> Address {
    read_metadata(e).asset_controller
}

pub fn read_is_active(e: &Env) -> bool {
    read_metadata(e).is_active
}

pub fn read_enforced(e: &Env) -> bool {
    read_metadata(e).enforced
}
