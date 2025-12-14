#![allow(unused)]

use anyhow::Result;
use axum::response::IntoResponse;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello2/John").await?.print().await?;

    Ok(())
}
