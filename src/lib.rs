//! hupasiya - Multi-agent session orchestrator
//!
//! This is the library crate for hupasiya, providing session management,
//! context management, and hannahanna integration.

pub mod cli;
pub mod config;
pub mod context;
pub mod error;
pub mod hn_client;
pub mod models;
pub mod orchestration;
pub mod session;

pub use error::{Error, Result};
