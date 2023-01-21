use std::sync::Arc;

use fedimint_api::config::{ApiEndpoint, ClientConfig, FederationId};
use fedimint_api::core::Decoder;
use fedimint_api::db::mem_impl::MemDatabase;
use fedimint_api::db::Database;
use fedimint_api::module::registry::ModuleDecoderRegistry;
use fedimint_storage::common::StorageModuleDecoder;
use fedimint_storage::config::StorageClientConfig;
use mint_client::api::{self, DynFederationApi};
use mint_client::storage::StorageClient;
use mint_client::utils::ClientContext;

pub fn create_client_module() -> StorageClient {
    let auth_pk = threshold_crypto::SecretKey::random().public_key();
    let client_config = ClientConfig {
        federation_name: "".to_string(),
        federation_id: FederationId(auth_pk),
        epoch_pk: threshold_crypto::SecretKey::random().public_key(),
        auth_pk,
        nodes: create_nodes(&[18174, 18184, 18194, 18204]),
        modules: [].into(),
    };

    StorageClient {
        config: StorageClientConfig { something: 1234 },
        context: Arc::new(ClientContext {
            api: DynFederationApi::from(api::WsFederationApi::from_config(&client_config)),
            db: Database::new(MemDatabase::new(), module_decode_stubs()),
            secp: secp256k1_zkp::Secp256k1::new(),
        }),
    }
}

fn create_nodes(ports: &[u32]) -> Vec<ApiEndpoint> {
    ports
        .into_iter()
        .map(|port| ApiEndpoint {
            url: url::Url::parse(&format!("ws://localhost:{}", port)).unwrap(),
            name: format!("server-{}", port).to_string(),
        })
        .collect::<Vec<_>>()
}

fn module_decode_stubs() -> ModuleDecoderRegistry {
    ModuleDecoderRegistry::from_iter([(3, Decoder::from_typed(StorageModuleDecoder))])
}
