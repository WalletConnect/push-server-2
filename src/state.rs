use {
    crate::{
        config::EnvConfig,
        stores::{client::ClientStore, tenant::TenantStore},
    },
    build_info::BuildInfo,
    std::sync::Arc,
};

pub struct AppState {
    pub config: EnvConfig,
    pub build_info: BuildInfo,
    pub store: StoreArc,
    pub tenant_store: TenantStoreArc,
}

build_info::build_info!(fn build_info);

pub type StoreArc = Arc<dyn ClientStore + Send + Sync + 'static>;
pub type TenantStoreArc = Arc<dyn TenantStore + Send + Sync + 'static>;

impl AppState {
    pub fn new(config: EnvConfig, store: StoreArc, tenant_store: TenantStoreArc) -> Self {
        let build_info: &BuildInfo = build_info();

        Self {
            config,
            build_info: build_info.clone(),
            store,
            tenant_store,
        }
    }
}
