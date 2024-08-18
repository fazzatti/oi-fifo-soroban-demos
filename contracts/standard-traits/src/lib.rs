#![no_std]

pub mod asset_controller {
    include!("./asset-controller/asset_controller.rs");
}

pub mod classic_wrapper {

    pub mod optional {
        include!("./classic-wrapper-interface/optional/classic_wrapper.rs");
    }

    pub mod enforced {
        include!("./classic-wrapper-interface/enforced/classic_wrapper.rs");
    }

    pub mod common {
        include!("./classic-wrapper-interface/common.rs");
    }
}
