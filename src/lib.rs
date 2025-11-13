//! hupasiya - Multi-agent session orchestrator
//!
//! This is the library crate for hupasiya, providing session management,
//! context management, and hannahanna integration.

// Allow dead code and enum variant names during development
#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]

pub mod ai_tool;
pub mod cli;
pub mod config;
pub mod context;
pub mod error;
pub mod hn_client;
pub mod models;
pub mod orchestration;
pub mod pr;
pub mod session;

pub use error::{Error, Result};
