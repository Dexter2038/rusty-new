#![allow(unused_imports)]
use crate::color::{EMBER_RED, FERRIS_TEAL, JET_BLACK, PURE_WHITE, RUST_ORANGE, STEEL_GRAY};
use inquire::ui::{RenderConfig, StyleSheet};

pub fn get_render_cfg() -> RenderConfig<'static> {
    let option_style = StyleSheet::default().with_fg(STEEL_GRAY.into());
    let answer_style = StyleSheet::default().with_fg(RUST_ORANGE.into());
    RenderConfig::default()
        .with_option(option_style)
        .with_answer(answer_style)
        .with_selected_option(Some(answer_style))
}
