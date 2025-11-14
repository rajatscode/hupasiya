//! hupasiya - Multi-agent session orchestrator
//!
//! This is the library crate for hupasiya, providing session management,
//! context management, and hannahanna integration.

pub mod activity;
pub mod ai_tool;
pub mod cli;
pub mod collaboration;
pub mod config;
pub mod context;
pub mod error;
pub mod hn_client;
pub mod models;
pub mod orchestration;
pub mod pr;
pub mod profiles;
pub mod progress;
pub mod session;
pub mod shepherd;
pub mod templates;
pub mod tutorial;
pub mod utilities;

pub use error::{Error, Result};
