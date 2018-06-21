//! Contains basic wrappers for the Erlang NIF api. Should not be used directly.
//!
//! While the nif_interface module should directly export unsafe nif helper functions,
//! this module should preform validation and make them (reasonably) safe and easy to
//! use from rust. This module should try to be as nonopinionated as possible, and
//! should try to stick as close as possible to the original C api.
//!
//! Making the apis nice to use from rust should be done in the root rustler crate.

pub mod atom;
pub mod binary;
pub mod check;
pub mod env;
pub mod exception;
pub mod list;
pub mod map;
pub mod nif_interface;
pub mod pid;
pub mod resource;
pub mod term;
pub mod tuple;

pub use self::nif_interface::enif_make_copy as copy_term;
