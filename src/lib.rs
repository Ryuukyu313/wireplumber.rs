#![doc(html_root_url = "https://arcnmx.github.io/wireplumber.rs/")]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

//! [WirePlumber](https://pipewire.pages.freedesktop.org/wireplumber/index.html) library bindings
//!
//! This crate provides a high-level interface to [PipeWire](https://pipewire.org/)'s [API](https://docs.pipewire.org/page_api.html)
//! via libwireplumber.
//!
//! # Upstream Documentation
//!
//! WirePlumber is a [GObject library](https://gtk-rs.org/), and its [C API documentation](https://pipewire.pages.freedesktop.org/wireplumber/c_api.html)
//! may also be helpful.

#[allow(unused_imports)]
mod auto;

pub use auto::*;
pub use ffi;

/// Export dependencies for use from macros
#[doc(hidden)]
pub mod lib {
	pub use glib;
	pub use gio;
}

pub type Result<T> = std::result::Result<T, glib::Error>;

pub mod pw;
pub mod prelude;

pub mod local;

mod error;

mod core;
pub use crate::core::*;

#[macro_use]
mod log;
pub use log::*;

mod object;
pub use object::*;

mod object_manager;
pub use object_manager::*;

mod plugin;
pub use plugin::*;

mod proxy;
pub use proxy::*;

mod iterator;
pub use iterator::*;

mod transition;
pub use transition::*;

mod node;
pub use node::*;

mod link;
pub use link::*;

mod port;
pub use port::*;

mod properties;
pub use properties::*;

mod spa;
pub use spa::*;

mod si;
pub use si::*;

mod interest;
pub use interest::*;

pub mod lua;

pub mod signals;
