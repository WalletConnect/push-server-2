use {super::context::ServerContext, test_context::test_context};

#[test_context(ServerContext)]
#[tokio::test]
async fn test_health(ctx: &mut ServerContext) {
    let response = reqwest::get(format!("http://{}/health", ctx.0.public_addr))
        .await
        .expect("Failed to call /health");

    assert!(response.status().is_success());

    let content = response.text().await.expect("Failed to parse response as string");
    let build = super::get_build_info();

    assert_eq!(content, format!("OK, push-server v{}", build.crate_info.version))
}