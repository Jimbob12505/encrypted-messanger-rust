use anyhow::Result;

/// Later this becomes QUIC/TLS. For now, just a stubbed start.
pub async fn start_server() -> Result<()> {
    tracing::info!("net::start_server() — server scaffold running");
    Ok(())
}

pub fn version() -> &'static str { "0.0.1" }

