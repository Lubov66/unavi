use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Features:");
    let feat_social = cfg!(feature = "social");
    info!("- social: {}", feat_social);
    let feat_web = cfg!(feature = "web");
    info!("- web: {}", feat_web);
    let feat_world = cfg!(feature = "world");
    info!("- world: {}", feat_world);

    if let Err(e) = unavi_server::start(unavi_server::ServerOptions::default()).await {
        error!(e);
    }
}
