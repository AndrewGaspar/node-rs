//! # node.js
//! Rust [stdweb](https://docs.rs/stdweb/*/stdweb/) bindings for the
//! [node.js](https://nodejs.org/en/) API when targetting WebAssembly.
//!
//! ## API Design
//! Modules are laid out in a hierarchy roughly mapping to the node.js API. Global objects are
//! available directly off the node_rs:: namespace.

#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate stdweb_derive;

use std::path::PathBuf;
use stdweb::unstable::TryInto;

pub(crate) fn js_private() -> &'static stdweb::Reference {
    static mut RUST_NODEJS_PRIVATE: Option<stdweb::Reference> = None;

    unsafe {
        if RUST_NODEJS_PRIVATE.is_none() {
            RUST_NODEJS_PRIVATE = js! {
                return global.RUST_NODEJS_PRIVATE = {};
            }.into_reference();
        }
    }

    unsafe { RUST_NODEJS_PRIVATE.as_ref() }.expect("RUST_NODEJS_PRIVATE not correctly initialized.")
}

pub mod child_process;
pub mod cluster;
pub mod timers;

mod process;
mod promise;

pub use process::Process;
pub use promise::{Promise, PromiseCallback};
pub use timers::{clear_timeout, set_timeout};

/// Returns the file path of the current script.
pub fn filename() -> PathBuf {
    PathBuf::from(
        js! { return __filename; }
            .into_string()
            .expect("__filename not found!"),
    )
}

/// Returns the directory of the current script.
pub fn dirname() -> PathBuf {
    PathBuf::from(
        js! { return __dirname; }
            .into_string()
            .expect("__dirname not found!"),
    )
}

/// Alias for `global.process`.
pub fn process() -> Process {
    (js! { return process; })
        .try_into()
        .expect("process global missing!")
}
