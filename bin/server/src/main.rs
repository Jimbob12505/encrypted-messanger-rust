use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    tracing::info!("server boot");

    net::start_server().await?;
    tracing::info!("server ok; version {}", net::version());

    Ok(())
}

