#![no_std]

mod admin;
mod asset;
mod contract;
mod data;
mod events;
mod quota;
mod rules;
mod storage_types;
mod test;
mod validations;

pub use crate::contract::AssetController;
