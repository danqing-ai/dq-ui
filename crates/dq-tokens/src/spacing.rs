pub const UNIT: f32 = 4.0;
pub const XS: f32 = UNIT;
pub const SM: f32 = UNIT * 2.0;
pub const MD: f32 = UNIT * 3.0;
pub const LG: f32 = UNIT * 4.0;
pub const XL: f32 = UNIT * 6.0;
pub const XXL: f32 = UNIT * 8.0;

pub const SIDEBAR_ITEM_HEIGHT: f32 = 28.0;
pub const BUTTON_HEIGHT_SM: f32 = 28.0;
pub const BUTTON_HEIGHT_MD: f32 = 32.0;
pub const BUTTON_HEIGHT_LG: f32 = 36.0;

/// Single-line control height — inputs, pick_list, icon buttons, control-row buttons.
pub const CONTROL_HEIGHT: f32 = 32.0;
pub const INPUT_HEIGHT: f32 = CONTROL_HEIGHT;
pub const ICON_BUTTON_SIZE: f32 = CONTROL_HEIGHT;
pub const SLIDER_HEIGHT: f32 = CONTROL_HEIGHT;

/// Horizontal inset inside bordered controls.
pub const CONTROL_PADDING_X: f32 = 12.0;
/// Gap between controls on the same row.
pub const CONTROL_ROW_GAP: f32 = SM;
/// Gap between stacked form blocks (label+control groups).
pub const CONTROL_BLOCK_GAP: f32 = MD;

pub const LOAD_BUTTON_WIDTH: f32 = 52.0;
pub const EDITOR_HEIGHT_PROMPT: f32 = 96.0;
pub const EDITOR_HEIGHT_NEGATIVE: f32 = 72.0;

/// Compact numeric fields in advanced panel.
pub const FIELD_WIDTH_SEED: f32 = 88.0;
pub const FIELD_WIDTH_STEPS: f32 = 64.0;

#[inline]
pub fn radius_group() -> f32 {
    crate::semantic::active_metrics().radius_group
}
#[inline]
pub fn radius_control() -> f32 {
    crate::semantic::active_metrics().radius_control
}
#[inline]
pub fn radius_control_sm() -> f32 {
    crate::semantic::active_metrics().radius_control_sm
}
#[inline]
pub fn radius_button() -> f32 {
    crate::semantic::active_metrics().radius_button
}
#[inline]
pub fn radius_input() -> f32 {
    crate::semantic::active_metrics().radius_input
}
#[inline]
pub fn row_gutter() -> f32 {
    crate::semantic::active_metrics().row_gutter
}

/// @deprecated use `radius_control_sm()`
pub const RADIUS_SM: f32 = 4.0;
/// @deprecated use `radius_control()`
pub const RADIUS_MD: f32 = 6.0;
/// @deprecated use `radius_group()`
pub const RADIUS_LG: f32 = 8.0;
