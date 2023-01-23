use std::sync::Arc;

use fedimint_api::{
    config::{ApiEndpoint, ClientConfig, FederationId},
    core::Decoder,
    db::{mem_impl::MemDatabase, Database},
    module::registry::ModuleDecoderRegistry,
};
use fedimint_storage::{common::StorageModuleDecoder, config::StorageClientConfig};
use mint_client::{
    api::{DynFederationApi, WsFederationApi},
    storage::StorageClient,
    utils::ClientContext,
};
use secp256k1::Secp256k1;

fn create_client_module() -> StorageClient {
    let auth_pk = threshold_crypto::SecretKey::random().public_key();
    let client_config = ClientConfig {
        federation_name: "".to_string(),
        federation_id: FederationId(auth_pk),
        epoch_pk: threshold_crypto::SecretKey::random().public_key(),
        auth_pk,
        nodes: vec![ApiEndpoint {
            url: url::Url::parse("ws://localhost:18174").unwrap(),
            name: "test".to_string(),
        }],
        modules: [].into(),
    };

    StorageClient {
        config: StorageClientConfig { something: 1234 },
        context: Arc::new(ClientContext {
            api: DynFederationApi::from(WsFederationApi::from_config(&client_config)),
            db: Database::new(
                MemDatabase::new(),
                ModuleDecoderRegistry::from_iter([(3, Decoder::from_typed(StorageModuleDecoder))]),
            ),
            secp: Secp256k1::new(),
        }),
    }
}

// this is a integrationtests that expects a running mit providing the storage-module. scripts/tmuxinator.sh has to be started first
#[tokio::test]
async fn test_store_and_retrieve() {
    let client = create_client_module();
    let fixture = "foo bar baz qu".as_bytes().to_vec();
    let res = client.store_data_raw(fixture.clone()).await;
    dbg!(&res);
    assert!(res.is_ok());
    let key = res.unwrap();

    let res_retrieve = client.retrieve_data_raw(key).await;
    assert!(res_retrieve.is_ok());
    assert_eq!(fixture, res_retrieve.unwrap());
}
