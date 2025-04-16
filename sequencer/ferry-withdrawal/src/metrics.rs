use lazy_static::lazy_static;

use prometheus::{
    opts, register_counter, register_counter_vec, register_gauge_vec, Counter, CounterVec, GaugeVec,
};

lazy_static! {
    pub static ref FERRIED: CounterVec = register_counter_vec!(
        opts!("ferried", "Ferried deposits amount per token"),
        &["token"]
    )
    .unwrap();
}

lazy_static! {
    pub static ref FERRIED_VOLUME: CounterVec =
        register_counter_vec!(opts!("volume", "Ferried volume per token"), &["token"]).unwrap();
}

lazy_static::lazy_static! {
    pub static ref FAILED_FERRY_ATTEMPTS: Counter = register_counter!(opts!(
        "failed_ferry_attempts",
        "Amount of unsuccessful ferry attempts",
    ))
    .unwrap();
}

lazy_static! {
    pub static ref TOKEN_BALANCE: GaugeVec = register_gauge_vec!(
        opts!("token_balance", "Account balance per token"),
        &["token"]
    )
    .unwrap();
}
