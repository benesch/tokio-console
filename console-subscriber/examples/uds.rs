use std::path::Path;
use std::time::Duration;
use tokio::{task, time};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = Path::new("./console-server");
    console_subscriber::ConsoleLayer::builder()
        .server_addr(addr)
        .init();
    info!("listening for console connections at {}", addr.display());
    task::Builder::default()
        .name("sleepy")
        .spawn(async move { time::sleep(Duration::from_secs(90)).await })
        .unwrap()
        .await?;

    Ok(())
}
