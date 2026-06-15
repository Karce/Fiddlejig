//! Autofisher-3000 — testable core.
//!
//! The fishing logic is a pure state machine ([`state::step`]) with no I/O, so it
//! is exhaustively unit-tested without a desktop. The binary ([`main`](../main.rs))
//! is the imperative shell that wires portal capture + detection + input around it.
//!
//! Later phases add `capture`, `detect`, `input`, `portal`, and `error` modules.

pub mod capture;
pub mod config;
pub mod detect;
pub mod frame;
pub mod nn;
pub mod portal;
pub mod state;
