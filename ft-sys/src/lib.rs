//! This crate is part of [ft-sdk](https://docs.rs/ft-sdk/) and provides the
//! system-level functionality. This crate should not be used directly, and
//! `ft-sdk` should be used.

extern crate self as ft_sys;

mod crypto;
#[cfg(feature = "postgres")]
mod diesel_pg;
#[cfg(feature = "sqlite")]
mod diesel_sqlite;
pub mod env;
pub mod http;
pub mod memory;

pub use crypto::{decrypt, encrypt};
pub use diesel_pg::PgConnection;
pub use diesel_sqlite::{SqliteConnection, SqliteTimestampz, Timestamptz};
pub use ft_sys_shared::{DecryptionError, UserData};

pub use env::now;
