#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn test() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:5000")?;

    // client.do_get("/api/auth/health_check").await?.print().await?;

    let response = client
        .do_post(
        "/api/auth/health_check", json!("anjay"))
        .await?;

    response.print().await?;
    Ok(())
}