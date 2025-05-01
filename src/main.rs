#![allow(dead_code)]

#[allow(unused_imports)]
use crate::color::{EMBER_RED, FERRIS_TEAL, JET_BLACK, PURE_WHITE, RUST_ORANGE, STEEL_GRAY};

use colored::{self, Colorize};
use config::get_render_cfg;
use inquire::Select;
use kinds::{ProjectType, webapp::WebAppConfig};

mod color;
mod config;
mod kinds;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}  {}\n    {}",
        "🦀".custom_color(RUST_ORANGE),
        "create-rust-app".custom_color(FERRIS_TEAL),
        "fast, idiomatic, async Rust scaffolding".custom_color(STEEL_GRAY)
    );

    let project_types = vec![
        ProjectType::WebApp(WebAppConfig::new()),
        ProjectType::CliApp,
        ProjectType::Library,
        ProjectType::Embedded,
        ProjectType::Game,
    ];

    Select::new("What type of project do you want to create?", project_types)
        .with_render_config(get_render_cfg())
        .prompt()?;

    Ok(())
}
