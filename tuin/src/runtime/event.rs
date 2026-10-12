//! # event
//! This module holds logic for events.

use super::Input;

/// # Event
/// Requires Clone and Debug.
pub trait Event: Clone + std::fmt::Debug + From<Input> {}
