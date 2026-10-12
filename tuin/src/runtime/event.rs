//! # event
//! This module holds logic for events.

/// # Event
/// Requires Clone and Debug.
pub trait Event: Clone + std::fmt::Debug {}
