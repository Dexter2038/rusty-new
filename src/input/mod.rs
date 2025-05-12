use inquire::Select;
use web::WebAppConfig;

use crate::config::get_render_cfg;

pub mod web;

#[derive(strum_macros::Display)]
pub enum ProjectConfig {
    WebApp(WebAppConfig),
    // CliApp,
    // Library,
    // Embedded,
    // Game,
}

impl ProjectConfig {
    pub fn new() -> Result<ProjectConfig, Box<dyn std::error::Error>> {
        // let kinds = vec![
        //     ProjectConfig::WebApp(WebAppConfig::new()),
        //     ProjectConfig::CliApp,
        //     ProjectConfig::Library,
        //     ProjectConfig::Embedded,
        //     ProjectConfig::Game,
        // ];
        // match Select::new("What type of project do you want to create?", kinds)
        //     .with_render_config(get_render_cfg())
        //     .prompt()?
        match ProjectConfig::WebApp(WebAppConfig::default()) {
            ProjectConfig::WebApp(_) => Ok(ProjectConfig::WebApp(WebAppConfig::build()?)),
            // ProjectConfig::CliApp => Ok(ProjectConfig::CliApp),
            // ProjectConfig::Library => Ok(ProjectConfig::Library),
            // ProjectConfig::Embedded => Ok(ProjectConfig::Embedded),
            // ProjectConfig::Game => Ok(ProjectConfig::Game),
        }
    }
}

pub trait BuildConfig {
    fn build() -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
