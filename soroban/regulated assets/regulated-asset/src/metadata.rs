use soroban_sdk::{Env, String};
use soroban_token_sdk::metadata::{TokenMetadata, Metadata};

pub fn read_decimal(e: &Env) -> u32 {
    let util = Metadata::new(e);
    util.get_metadata().decimal
}

pub fn read_name(e: &Env) -> String {
    let util = Metadata::new(e);
    util.get_metadata().name
}

pub fn read_symbol(e: &Env) -> String {
    let util = Metadata::new(e);
    util.get_metadata().symbol
}

pub fn write_metadata(e: &Env, metadata: TokenMetadata) {
    let util = Metadata::new(e);
    util.set_metadata(&metadata);
}   