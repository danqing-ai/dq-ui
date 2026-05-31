//! Linear-inspired design tokens for Iced widgets.

pub mod color;
pub mod generate;
pub mod layout;
pub mod semantic;
pub mod spacing;
pub mod typography;

pub use color::*;
pub use generate::{generate, ThemeInput};
pub use layout::*;
pub use semantic::{SemanticPalette, LINEAR_DARK};
pub use spacing::*;
pub use typography::*;
