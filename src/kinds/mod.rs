use inquire::Select;
use webapp::WebAppConfig;

use crate::config::get_render_cfg;

pub mod webapp;

#[derive(strum_macros::Display)]
pub enum ProjectKind {
    WebApp(WebAppConfig),
    CliApp,
    Library,
    Embedded,
    Game,
}

impl ProjectKind {
    pub fn new() -> Result<ProjectKind, Box<dyn std::error::Error>> {
        // let kinds = vec![
        //     ProjectKind::WebApp(WebAppConfig::new()),
        //     ProjectKind::CliApp,
        //     ProjectKind::Library,
        //     ProjectKind::Embedded,
        //     ProjectKind::Game,
        // ];
        // match Select::new("What type of project do you want to create?", kinds)
        //     .with_render_config(get_render_cfg())
        //     .prompt()?
        match ProjectKind::WebApp(WebAppConfig::default()) {
            ProjectKind::WebApp(_) => Ok(ProjectKind::WebApp(WebAppConfig::new()?)),
            ProjectKind::CliApp => Ok(ProjectKind::CliApp),
            ProjectKind::Library => Ok(ProjectKind::Library),
            ProjectKind::Embedded => Ok(ProjectKind::Embedded),
            ProjectKind::Game => Ok(ProjectKind::Game),
        }
    }
}
