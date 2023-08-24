use {
    super::DATABASE_URL,
    push_server::config::EnvConfig,
    std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener},
    tokio::{
        runtime::Handle,
        sync::broadcast,
        time::{sleep, Duration},
    },
};

#[cfg(feature = "multitenant")]
use super::TENANT_DATABASE_URL;

pub struct PushServer {
    pub public_addr: SocketAddr,
    shutdown_signal: broadcast::Sender<()>,
    is_shutdown: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl PushServer {
    pub async fn start() -> Self {
        let public_port = get_random_port();
        let config = EnvConfig {
            port: public_port,
            public_url: "".to_string(),
            database_url: DATABASE_URL.to_string(),
            #[cfg(feature = "multitenant")]
            tenant_database_url: TENANT_DATABASE_URL.to_string(),
        };
        let (public_addr, signal, is_shutdown) = start_server(config).await;

        Self {
            public_addr,
            shutdown_signal: signal,
            is_shutdown,
        }
    }

    pub async fn shutdown(&mut self) {
        if self.is_shutdown {
            return;
        }
        self.is_shutdown = true;
        let _ = self.shutdown_signal.send(());
        wait_for_server_to_shutdown(self.public_addr.port())
            .await
            .unwrap();
    }
}

async fn start_server(
    config: EnvConfig,
) -> (
    SocketAddr,
    broadcast::Sender<()>,
    bool,
) {
    let rt = Handle::current();
    let port = config.port.clone();
    let public_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);

    let (signal, shutdown) = broadcast::channel(1);

    std::thread::spawn(move || {
        rt.block_on(async move { push_server::bootstrap(shutdown, config).await })
            .unwrap();
    });

    if let Err(e) = wait_for_server_to_start(port).await {
        panic!("Failed to start server with error: {e:?}")
    }

    (public_addr, signal, false)
}

// Finds a free port.
fn get_random_port() -> u16 {
    use std::sync::atomic::{AtomicU16, Ordering};

    static NEXT_PORT: AtomicU16 = AtomicU16::new(9000);

    loop {
        let port = NEXT_PORT.fetch_add(1, Ordering::SeqCst);

        if is_port_available(port) {
            return port;
        }
    }
}

fn is_port_available(port: u16) -> bool {
    TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port)).is_ok()
}

async fn wait_for_server_to_shutdown(port: u16) -> Result<(), super::ContextError> {
    let poll_fut = async {
        while !is_port_available(port) {
            sleep(Duration::from_millis(10)).await;
        }
    };

    Ok(tokio::time::timeout(Duration::from_secs(3), poll_fut).await?)
}

async fn wait_for_server_to_start(port: u16) -> Result<(), super::ContextError> {
    let poll_fut = async {
        while is_port_available(port) {
            sleep(Duration::from_millis(10)).await;
        }
    };

    Ok(tokio::time::timeout(Duration::from_secs(5), poll_fut).await?)
}