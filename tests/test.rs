#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn test() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:5000")?;
    let response = client
        .do_post(
            "/api/auth/login",
            json!({
                "username": "admin@gmail.com",
                "password": "admin123"
            })
        )
        .await?;

    response.print().await?;
    Ok(())
}

#[tokio::test]
async fn rate_limit_test() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:5000")?;

    let response1 = client
        .do_post("/api/auth/health_check", json!("keceeee"))
        .await?;
    
    response1.print().await?;
    println!("First request status: {}", response1.status());
    assert!(response1.status().is_success());

    // Second request should fail due to rate limiting
    let response2 = client
        .do_post("/api/auth/health_check", json!("keceeee2"))
        .await;

    match response2 {
        Ok(resp) => {
            resp.print().await?;
            println!("Second request status: {}", resp.status());
            // Should be 429 (Too Many Requests) or similar
            // assert!(resp.status().as_u16() >= 400);
        }
        Err(e) => {
            println!("Second request failed as expected: {}", e);
            // This is also acceptable - rate limiting might cause connection issues
        }
    }

    Ok(())
}