use test_context::test_context;
use uuid::Uuid;
use super::context::StoreContext;

#[test_context(StoreContext)]
#[tokio::test]
async fn test_create_client(ctx: &mut StoreContext) {
    let client_id = Uuid::new_v4().to_string();

    // let _client = ctx.store.create_or_update_client().await.expect("Failed to create client");
    //
    // let client = ctx.store.get_client(client_id).await.expect("Failed to fetch the created client");
    // assert_eq!(client.id, client_id)
}