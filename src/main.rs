#![allow(dead_code)]

#[allow(unused_imports)]
use crate::color::{EMBER_RED, FERRIS_TEAL, JET_BLACK, PURE_WHITE, RUST_ORANGE, STEEL_GRAY};

use colored::{self, Colorize};
use kinds::ProjectKind;

mod color;
mod config;
mod kinds;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}  {}\n    {}",
        "🦀".custom_color(RUST_ORANGE),
        "rise (rust initial setup engine)".custom_color(FERRIS_TEAL),
        "fast, idiomatic, async Rust scaffolding".custom_color(STEEL_GRAY)
    );

    let project_type = ProjectKind::new()?;

    println!("{project_type}");

    Ok(())
}
