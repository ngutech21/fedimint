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
        nodes: create_nodes(),
        modules: [].into(),
    };
    let ws_api = DynFederationApi::from(api::WsFederationApi::from_config(&client_config));

    let mem_db = Database::new(MemDatabase::new(), module_decode_stubs());

    StorageClient {
        config: StorageClientConfig { something: 1234 },
        context: Arc::new(ClientContext {
            api: ws_api,
            db: mem_db,
            secp: secp256k1_zkp::Secp256k1::new(),
        }),
    }
}

fn create_nodes() -> Vec<ApiEndpoint> {
    vec![
        ApiEndpoint {
            url: url::Url::parse("ws://localhost:18174").unwrap(),
            name: "server-0".to_string(),
        },
        ApiEndpoint {
            url: url::Url::parse("ws://localhost:18184").unwrap(),
            name: "server-1".to_string(),
        },
        ApiEndpoint {
            url: url::Url::parse("ws://localhost:18194").unwrap(),
            name: "server-2".to_string(),
        },
        ApiEndpoint {
            url: url::Url::parse("ws://localhost:18204").unwrap(),
            name: "server-3".to_string(),
        },
    ]
}

fn module_decode_stubs() -> ModuleDecoderRegistry {
    ModuleDecoderRegistry::from_iter([
        // (
        //     LEGACY_HARDCODED_INSTANCE_ID_LN,
        //     Decoder::from_typed(LightningModuleDecoder),
        // ),
        // (
        //     LEGACY_HARDCODED_INSTANCE_ID_WALLET,
        //     Decoder::from_typed(WalletModuleDecoder),
        // ),
        (3, Decoder::from_typed(StorageModuleDecoder)),
    ])
}
