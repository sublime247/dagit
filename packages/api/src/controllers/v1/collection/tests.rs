use super::*;
use crate::{
    controllers::v1::collection,
    dagit_tests::{TestContext, setup},
};
use models::v1::collections::CollectionQuery;

const IMAGE_URL: &str =
    "https://metadata.dev.dagit.club/images/72a11429-20c0-4d62-8cde-ff3d4d5dc0bb";

async fn create_collection() -> Collection {
    let TestContext { endpoint, user, .. } = setup().await.unwrap();
    let client = Collection::get_client(&endpoint);
    let result = client
        .create(
            "TEST_Collection".to_string(),
            "TEDESCRIPTION".to_string(),
            None,
            IMAGE_URL.to_string(),
            IMAGE_URL.to_string(),
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to create collection: {:?}",
        result.err()
    );
    let result = result.unwrap();
    result
}
#[tokio::test]
async fn test_create_collection() {
    let TestContext { endpoint, user, .. } = setup().await.unwrap();
    let _agit_id = user.agits.first().unwrap().id;
    let client = Collection::get_client(&endpoint);
    let result = client
        .create(
            "TEST_Collection".to_string(),
            "TEDESCRIPTION".to_string(),
            None,
            IMAGE_URL.to_string(),
            IMAGE_URL.to_string(),
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to create collection: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_query_collection() {
    let TestContext { endpoint, user, .. } = setup().await.unwrap();
    let _agit_id = user.agits.first().unwrap().id;

    let client = Collection::get_client(&endpoint);

    let param = CollectionQuery {
        bookmark: None,
        size: 10,
    };
    let result = client.query(param).await;

    assert!(
        result.is_ok(),
        "Failed to query collection: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_delete_collection() {
    let TestContext { endpoint, user, .. } = setup().await.unwrap();
    let _agit_id = user.agits.first().unwrap().id;
    let client = Collection::get_client(&endpoint);
    let collection = create_collection().await;
    let result = client.delete(collection.id).await;

    assert!(
        result.is_ok(),
        "Failed to delete collection: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_update_collection() {
    let TestContext { endpoint, user, .. } = setup().await.unwrap();
    let _agit_id = user.agits.first().unwrap().id;
    let client = Collection::get_client(&endpoint);
    let collection = create_collection().await;
    let result = client
        .update(
            collection.id,
            collection.title,
            collection.description,
            None,
            collection.banner_url,
            collection.logo_url,
            true,
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to update collection: {:?}",
        result.err()
    );
    let result = result.unwrap();
    assert_eq!(result.authorized, true);
}
