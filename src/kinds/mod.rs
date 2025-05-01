use inquire::Select;
use webapp::WebAppConfig;

pub mod embedded;
pub mod gamedev;
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
        let kinds = vec![
            ProjectKind::WebApp(WebAppConfig::new()),
            ProjectKind::CliApp,
            ProjectKind::Library,
            ProjectKind::Embedded,
            ProjectKind::Game,
        ];
        match Select::new("What type of project do you want to create?", kinds).prompt()? {
            ProjectKind::WebApp(mut config) => {
                config.populate();
                Ok(ProjectKind::WebApp(config))
            }
            ProjectKind::CliApp => Ok(ProjectKind::CliApp),
            ProjectKind::Library => Ok(ProjectKind::Library),
            ProjectKind::Embedded => Ok(ProjectKind::Embedded),
            ProjectKind::Game => Ok(ProjectKind::Game),
        }
    }
}
