use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> push_server::prelude::Result<()> {
    let logger = push_server::log::Logger::init().expect("Failed to start logging");

    let (_signal, shutdown) = broadcast::channel::<()>(1);
    dotenv::dotenv().ok();

    let config = push_server::config::get_config().expect(
        "Failed to load configuration from environment variables, please check the documentation \
         and try again.",
    );

    let res = push_server::bootstrap(shutdown, config).await;

    logger.stop();

    res
}
