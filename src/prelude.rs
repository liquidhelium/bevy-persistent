//! Preludes of the crate.

pub(crate) use crate::builder::PersistentBuilder;
pub(crate) use bevy::{
    log,
    prelude::*,
};
pub(crate) use serde::{
    de::DeserializeOwned,
    Serialize,
};
pub(crate) use std::{
    io::Write,
    ops::Deref,
    path::{
        Path,
        PathBuf,
    },
};

pub use crate::{
    persistent::Persistent,
    storage::StorageFormat,
};
