use {crate::config::EnvConfig, build_info::BuildInfo};

pub struct AppState {
    pub config: EnvConfig,
    pub build_info: BuildInfo,
}

build_info::build_info!(fn build_info);

impl AppState {
    pub fn new(config: EnvConfig) -> Self {
        let build_info: &BuildInfo = build_info();

        Self {
            config,
            build_info: build_info.clone(),
        }
    }
}
