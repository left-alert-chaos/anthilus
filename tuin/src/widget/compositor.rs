//! # compositor
//! This module holds rendering logic for compositing widgets.

/// # CompositorType
/// This enum differentiates between a vertical or horizontal compositor.
#[derive(Eq, PartialEq, Clone)]
pub enum CompositorType {
    Row,
    Column,
}
