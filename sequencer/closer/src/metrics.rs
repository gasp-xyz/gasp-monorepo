use prometheus::{opts, register_gauge, Gauge};

lazy_static::lazy_static! {
    pub static ref CLOSED_WITHDRAWAL: Gauge = register_gauge!(opts!(
        "closed_withdrawals",
        "Amount of successfully closed withdrawals",
    ))
    .unwrap();
}

lazy_static::lazy_static! {
    pub static ref FAILED_CLOSE_ATTEMPTS: Gauge = register_gauge!(opts!(
        "failed_close_attempt",
        "Amount of failed attempt when closing",
    ))
    .unwrap();
}

lazy_static::lazy_static! {
    pub static ref PENDING_WITHDRAWALS: Gauge = register_gauge!(opts!(
        "pending_withdrawals",
        "amount of withdrawals ready to be closed",
    ))
    .unwrap();
}
