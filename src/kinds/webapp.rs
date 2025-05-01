use inquire::{MultiSelect, Select};

pub struct WebAppConfig {
    pub kind: Option<WebApp>,
    pub backend: Option<Backend>,
    pub frontend: Option<Frontend>,
    pub database: Option<Database>,
    pub deployment: Option<Deployment>,
    pub testings: Vec<Testing>,
    pub features: Vec<Features>,
}

impl WebAppConfig {
    pub fn new() -> WebAppConfig {
        WebAppConfig {
            kind: None,
            backend: None,
            frontend: None,
            database: None,
            deployment: None,
            testings: Vec::new(),
            features: Vec::new(),
        }
    }

    pub fn get_backends_options(&self) -> Option<Vec<Backend>> {
        if self.kind.clone().unwrap() == WebApp::Frontend {
            return None;
        }
        vec![
            Backend::Actix,
            Backend::Axum,
            Backend::Rocket,
            Backend::Warp,
            Backend::Tide,
            Backend::Salvo,
            Backend::Gotham,
        ]
        .into()
    }
    pub fn get_frontends_options(&self) -> Option<Vec<Frontend>> {
        if self.kind.clone().unwrap() == WebApp::Backend {
            return None;
        }
        let common_frontends = vec![
            Frontend::Svelte,
            Frontend::React,
            Frontend::Vue,
            Frontend::Minijinja,
            Frontend::Handlebars,
        ];

        match self.backend {
            Some(_) => {
                let backend_specific_frontends =
                    vec![Frontend::Yew, Frontend::Tera, Frontend::Askama];
                Some([common_frontends, backend_specific_frontends].concat())
            }
            None => {
                let no_backend_frontends = vec![Frontend::Leptos, Frontend::Seed, Frontend::Sauron];
                Some([common_frontends, no_backend_frontends].concat())
            }
        }
    }

    #[allow(clippy::all)]
    pub fn get_databases_options(&self) -> Option<Vec<Database>> {
        match self.backend {
            Some(_) => Some(vec![
                Database::SQLx,
                Database::Diesel,
                Database::SeaORM,
                Database::Sled,
            ]),
            None => None,
        }
    }

    pub fn get_deployment_options(&self) -> Option<Vec<Deployment>> {
        match (self.backend.clone(), self.frontend.clone()) {
            (
                None,
                Some(
                    Frontend::Tera | Frontend::Askama | Frontend::Minijinja | Frontend::Handlebars,
                ),
            ) => None,
            _ => Some(vec![
                Deployment::Docker,
                Deployment::Kubernetes,
                Deployment::Serverless,
                Deployment::SelfHosted,
            ]),
        }
    }

    #[allow(clippy::all)]
    pub fn get_testing_options(&self) -> Option<Vec<Testing>> {
        match self.backend {
            Some(_) => Some(vec![Testing::Api, Testing::SQLx, Testing::Mock]),
            None => None,
        }
    }

    #[allow(clippy::all)]
    pub fn get_features_options(&self) -> Option<Vec<Features>> {
        match self.backend {
            Some(_) => Some(vec![
                Features::WebSockets,
                Features::gRPC,
                Features::GraphQL,
                Features::RateLimiting,
            ]),
            None => None,
        }
    }

    pub fn set_kind(&mut self, kind: WebApp) {
        self.kind = Some(kind);
    }

    pub fn set_backend(&mut self, backend: Backend) {
        self.backend = Some(backend);
    }

    pub fn set_frontend(&mut self, frontend: Frontend) {
        self.frontend = Some(frontend);
    }

    pub fn set_database(&mut self, database: Database) {
        self.database = Some(database);
    }

    pub fn set_deployment(&mut self, deployment: Deployment) {
        self.deployment = Some(deployment);
    }

    pub fn add_testings(&mut self, testings: Vec<Testing>) {
        self.testings.extend(testings);
    }

    pub fn add_features(&mut self, feature: Vec<Features>) {
        self.features.extend(feature);
    }

    pub fn populate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let kind = Select::new(
            "Select a webapp kind",
            vec![WebApp::Backend, WebApp::Frontend, WebApp::Fullstack],
        )
        .prompt()?;
        self.set_kind(kind);

        if let Some(backends) = self.get_backends_options() {
            let backend = Select::new("Select a backend", backends).prompt()?;

            self.set_backend(backend);
        }

        if let Some(frontends) = self.get_frontends_options() {
            let frontend = Select::new("Select a frontend tech", frontends).prompt()?;

            self.set_frontend(frontend);
        }

        if let Some(databases) = self.get_databases_options() {
            let database = Select::new("Select a database", databases).prompt()?;

            self.set_database(database);
        }

        if let Some(deployments) = self.get_deployment_options() {
            let deployment = Select::new("Select a deployment option", deployments).prompt()?;

            self.set_deployment(deployment);
        }

        if let Some(testings) = self.get_testing_options() {
            let testings = MultiSelect::new("Select testing options", testings).prompt()?;

            self.add_testings(testings);
        }

        if let Some(features) = self.get_features_options() {
            let features = MultiSelect::new("Select features", features).prompt()?;

            self.add_features(features);
        }
        Ok(())
    }
}

#[derive(strum_macros::Display, PartialEq, Clone)]
pub enum WebApp {
    Backend,
    Frontend,
    Fullstack,
}

#[derive(Clone, strum_macros::Display)]
pub enum Backend {
    Actix,
    Axum,
    Rocket,
    Warp,
    Tide,
    Salvo,
    Gotham,
}

#[derive(Clone, strum_macros::Display)]
pub enum Frontend {
    Yew,
    Leptos,
    Seed,
    Svelte,
    React,
    Vue,
    Sauron,
    Minijinja,
    Tera,
    Askama,
    Handlebars,
}

#[derive(strum_macros::Display)]
pub enum Database {
    SQLx,
    Diesel,
    SeaORM,
    Sled,
}

#[derive(strum_macros::Display)]
pub enum Deployment {
    Docker,
    Kubernetes,
    Serverless,
    SelfHosted,
}

#[derive(strum_macros::Display)]
pub enum Testing {
    Api,
    SQLx,
    Mock,
}

#[derive(strum_macros::Display)]
#[allow(non_camel_case_types)]
pub enum Features {
    WebSockets,
    gRPC,
    GraphQL,
    RateLimiting,
}
