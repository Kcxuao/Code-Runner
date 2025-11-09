use bollard::Docker;
use once_cell::sync::OnceCell;
use std::time::Duration;

use crate::config::Config;

static DOCKER_CLIENT: OnceCell<Docker> = OnceCell::new();

pub fn init_docker(config: &Config) -> anyhow::Result<()> {
    let _connect_timeout = Duration::from_secs(config.docker.connect_timeout);
    let _keepalive = Duration::from_secs(config.docker.keepalive_secs);

    let docker = Docker::connect_with_http(
        &config.docker.host,
        config.docker.request_timeout,
        bollard::API_DEFAULT_VERSION,
    )?;

    DOCKER_CLIENT
        .set(docker)
        .map_err(|_| anyhow::anyhow!("Docker 客户端已初始化"))?;

    println!("🐳 Connected to Docker at: {}", config.docker.host);

    Ok(())
}

pub fn get_docker() -> &'static Docker {
    DOCKER_CLIENT.get().expect("Docker client not initialized")
}
