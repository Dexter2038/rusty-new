#![allow(dead_code)]

#[allow(unused_imports)]
use crate::color::{EMBER_RED, FERRIS_TEAL, JET_BLACK, PURE_WHITE, RUST_ORANGE, STEEL_GRAY};

use colored::{self, Colorize};
use inquire::{
    Select,
    ui::{RenderConfig, StyleSheet},
};

mod color;

pub enum ProjectType {
    WebApp,
    CliApp,
    Library,
    Embedded,
    Game,
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ProjectType::WebApp => "Web App",
            ProjectType::CliApp => "CLI App",
            ProjectType::Library => "Library",
            ProjectType::Embedded => "Embedded",
            ProjectType::Game => "Game",
        };
        write!(f, "{}", s)
    }
}

fn get_render_cfg() -> RenderConfig<'static> {
    let option_style = StyleSheet::default().with_fg(STEEL_GRAY.into());
    let answer_style = StyleSheet::default().with_fg(RUST_ORANGE.into());
    RenderConfig::default()
        .with_option(option_style)
        .with_answer(answer_style)
        .with_selected_option(Some(answer_style))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}  {}\n    {}",
        "🦀".custom_color(RUST_ORANGE),
        "create-rust-app".custom_color(FERRIS_TEAL),
        "fast, idiomatic, async Rust scaffolding".custom_color(STEEL_GRAY)
    );

    let project_types = vec![
        ProjectType::WebApp,
        ProjectType::CliApp,
        ProjectType::Library,
        ProjectType::Embedded,
        ProjectType::Game,
    ];

    let result = Select::new("What type of project do you want to create?", project_types)
        .with_vim_mode(true)
        .with_render_config(get_render_cfg())
        .prompt()?;
    println!("{}", result);

    Ok(())
}
