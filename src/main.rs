#![allow(dead_code)]

#[allow(unused_imports)]
use crate::color::{EMBER_RED, FERRIS_TEAL, JET_BLACK, PURE_WHITE, RUST_ORANGE, STEEL_GRAY};

use colored::{self, Colorize};
use inquire::{
    Select,
    ui::{RenderConfig, StyleSheet},
};

mod color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}  {}\n    {}",
        "🦀".custom_color(RUST_ORANGE),
        "create-rust-app".custom_color(FERRIS_TEAL),
        "fast, idiomatic, async Rust scaffolding".custom_color(STEEL_GRAY)
    );

    let project_types = vec!["Web App", "Cli App", "Library", "Embedded", "Game"];

    let option_style = StyleSheet::default().with_fg(STEEL_GRAY.into());
    let answer_style = StyleSheet::default().with_fg(RUST_ORANGE.into());
    let render_cfg = RenderConfig::default()
        .with_option(option_style)
        .with_answer(answer_style)
        .with_selected_option(Some(answer_style));
    let result = Select::new("What type of project do you want to create?", project_types)
        .with_vim_mode(true)
        .with_render_config(render_cfg)
        .prompt()?;
    println!("{}", result);

    Ok(())
}
