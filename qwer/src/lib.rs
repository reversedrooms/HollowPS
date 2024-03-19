#![allow(incomplete_features)]
#![feature(specialization)]

#[cfg(feature = "collection")]
mod collection;
#[cfg(feature = "fastoct")]
mod fastoct;
#[cfg(feature = "protocol")]
mod protocol;

#[cfg(feature = "collection")]
pub use collection::*;
#[cfg(feature = "fastoct")]
pub use fastoct::*;
#[cfg(feature = "protocol")]
pub use protocol::*;
