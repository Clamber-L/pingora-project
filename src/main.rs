use anyhow::Result;
use pingora::prelude::*;
use pingora_project::SimpleProxy;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

const PROXY_ADDR: &'static str = "0.0.0.0:9000";

fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(Layer::new().with_filter(LevelFilter::TRACE))
        .init();

    tracing::info!("Starting server on {}", PROXY_ADDR);

    let mut server = Server::new(None)?;
    server.bootstrap();

    let sp = SimpleProxy {};
    let mut proxy = http_proxy_service(&server.configuration, sp);
    proxy.add_tcp(PROXY_ADDR);
    server.add_service(proxy);

    server.run_forever();
}
