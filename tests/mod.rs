use std::sync::Mutex;

lazy_static::lazy_static! {
    /// ENV_MUTEX should always be locked before editing *or* accessing env vars within tests
    static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
}

mod unit;

#[cfg(feature = "functional_tests")]
mod functional;