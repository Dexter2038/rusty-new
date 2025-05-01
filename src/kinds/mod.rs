use webapp::WebAppConfig;

pub mod embedded;
pub mod gamedev;
pub mod webapp;

pub enum ProjectType {
    WebApp(WebAppConfig),
    CliApp,
    Library,
    Embedded,
    Game,
}

impl std::fmt::Display for ProjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ProjectType::WebApp(_) => "Web App",
            ProjectType::CliApp => "CLI App",
            ProjectType::Library => "Library",
            ProjectType::Embedded => "Embedded",
            ProjectType::Game => "Game",
        };
        write!(f, "{}", s)
    }
}
