use build_info::BuildInfo;

mod context;
mod health;


build_info::build_info!(fn build_info);
pub fn get_build_info() -> BuildInfo {
    let info: &BuildInfo = build_info();
    info.clone()
}