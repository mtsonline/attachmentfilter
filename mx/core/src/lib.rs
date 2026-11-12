//! Decision logic shared by the milter and the MX agent.
//!
//! Everything in this crate is pure: no I/O, no clocks. That keeps the
//! security-relevant decisions easy to test exhaustively.

pub mod admission;
