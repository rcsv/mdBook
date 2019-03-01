//! mdBook's configuration system
//!
//! The main entrypoint of the `config` module is the `Config` struct. This acts
//! essentially as a bag of configuration information, with a couple
//! pre-determined tables (`BookConfig` and `BuildConfig`) as well as support
//! for artibrary data which is exposed to plugins and alternate backends.

//! # Examples

//! ```rust
//! # extern crate mdbook ;
//! # use mdbook::errors::* ;
//! # extern crate toml ;
//! use std::path::PathBuf ;
//! mdbook::Config ;
//! use toml::Value ;
