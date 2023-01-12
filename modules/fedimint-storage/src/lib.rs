use std::collections::{BTreeMap, HashSet};
use std::fmt;

use async_trait::async_trait;
use common::StorageModuleDecoder;
use fedimint_api::cancellable::Cancellable;
use fedimint_api::config::TypedServerModuleConsensusConfig;
use fedimint_api::config::{
    ClientModuleConfig, ConfigGenParams, DkgPeerMsg, ModuleConfigGenParams, ServerModuleConfig,
    TypedServerModuleConfig,
};
use fedimint_api::core::{Decoder, ModuleInstanceId, ModuleKind};
use fedimint_api::db::{Database, DatabaseTransaction};
use fedimint_api::encoding::{Decodable, Encodable};
use fedimint_api::module::__reexports::serde_json;
use fedimint_api::module::audit::Audit;
use fedimint_api::module::interconnect::ModuleInterconect;
use fedimint_api::module::{
    api_endpoint, ApiEndpoint, InputMeta, ModuleError, ModuleInit, TransactionItemAmount,
};
use fedimint_api::net::peers::MuxPeerConnections;
use fedimint_api::server::DynServerModule;
use fedimint_api::task::TaskGroup;
use fedimint_api::{plugin_types_trait_impl, OutPoint, PeerId, ServerModule};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::{StorageConfig, StorageConfigConsensus, StorageConfigPrivate};

pub mod common;
pub mod config;
pub mod db;

const KIND: ModuleKind = ModuleKind::from_static_str("storage");

use db::ExampleKey;
use db::ExampleValue;

#[derive(Debug)]
pub struct StorageModule {
    pub cfg: StorageConfig,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, Encodable, Decodable)]
pub struct StorageOutputConfirmation;

#[derive(Debug, Clone)]
pub struct StorageVerificationCache;

pub struct StorageConfigGenerator;

#[async_trait]
impl ModuleInit for StorageConfigGenerator {
    async fn init(
        &self,
        cfg: ServerModuleConfig,
        _db: Database,
        _task_group: &mut TaskGroup,
    ) -> anyhow::Result<DynServerModule> {
        Ok(StorageModule::new(cfg.to_typed()?).into())
    }

    fn module_kind(&self) -> ModuleKind {
        ModuleKind::from_static_str("storage")
    }

    fn decoder(&self) -> Decoder {
        Decoder::from_typed(StorageModuleDecoder)
    }

    fn trusted_dealer_gen(
        &self,
        peers: &[PeerId],
        params: &ConfigGenParams,
    ) -> BTreeMap<PeerId, ServerModuleConfig> {
        let _params = params
            .get::<StorageConfigGenParams>()
            .expect("Invalid storage params");

        let mint_cfg: BTreeMap<_, StorageConfig> = peers
            .iter()
            .map(|&peer| {
                let config = StorageConfig {
                    private: StorageConfigPrivate {
                        something_private: 3,
                    },
                    consensus: StorageConfigConsensus { something: 1 },
                };
                (peer, config)
            })
            .collect();

        mint_cfg
            .into_iter()
            .map(|(k, v)| (k, v.to_erased()))
            .collect()
    }

    async fn distributed_gen(
        &self,
        _connections: &MuxPeerConnections<ModuleInstanceId, DkgPeerMsg>,
        _our_id: &PeerId,
        _instance_id: ModuleInstanceId,
        _peers: &[PeerId],
        params: &ConfigGenParams,
        _task_group: &mut TaskGroup,
    ) -> anyhow::Result<Cancellable<ServerModuleConfig>> {
        // FIXME
        // let _params = params
        //     .get::<StorageConfigGenParams>()
        //     .expect("Invalid storage params");

        let server = StorageConfig {
            private: StorageConfigPrivate {
                something_private: 3,
            },
            consensus: StorageConfigConsensus { something: 2 },
        };

        Ok(Ok(server.to_erased()))
    }

    fn to_client_config(&self, config: ServerModuleConfig) -> anyhow::Result<ClientModuleConfig> {
        Ok(config
            .to_typed::<StorageConfig>()?
            .consensus
            .to_client_config())
    }

    fn to_client_config_from_consensus_value(
        &self,
        config: serde_json::Value,
    ) -> anyhow::Result<ClientModuleConfig> {
        Ok(serde_json::from_value::<StorageConfigConsensus>(config)?.to_client_config())
    }

    fn validate_config(&self, identity: &PeerId, config: ServerModuleConfig) -> anyhow::Result<()> {
        config
            .to_typed::<StorageConfig>()?
            .validate_config(identity)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfigGenParams {
    pub important_param: u64,
}

impl ModuleConfigGenParams for StorageConfigGenParams {
    const MODULE_NAME: &'static str = "storage";
}

#[derive(
    Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, Encodable, Decodable, Default,
)]
pub struct StorageInput;

impl fmt::Display for StorageInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StorageInput")
    }
}

#[derive(
    Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, Encodable, Decodable, Default,
)]
pub struct StorageOutput;

impl fmt::Display for StorageOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StorageOutput")
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, Encodable, Decodable)]
pub struct StorageOutputOutcome;

impl fmt::Display for StorageOutputOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StorageOutputOutcome")
    }
}

impl fmt::Display for StorageOutputConfirmation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StorageOutputConfirmation")
    }
}

#[async_trait]
impl ServerModule for StorageModule {
    const KIND: ModuleKind = KIND;
    type Decoder = StorageModuleDecoder;
    type Input = StorageInput;
    type Output = StorageOutput;
    type OutputOutcome = StorageOutputOutcome;
    type ConsensusItem = StorageOutputConfirmation;
    type VerificationCache = StorageVerificationCache;

    fn decoder(&self) -> Self::Decoder {
        StorageModuleDecoder
    }

    async fn await_consensus_proposal(&self, _dbtx: &mut DatabaseTransaction<'_>) {}

    async fn consensus_proposal(
        &self,
        _dbtx: &mut DatabaseTransaction<'_>,
    ) -> Vec<Self::ConsensusItem> {
        vec![]
    }

    async fn begin_consensus_epoch<'a, 'b>(
        &'a self,
        _dbtx: &mut DatabaseTransaction<'b>,
        _consensus_items: Vec<(PeerId, Self::ConsensusItem)>,
    ) {
    }

    fn build_verification_cache<'a>(
        &'a self,
        _inputs: impl Iterator<Item = &'a Self::Input> + Send,
    ) -> Self::VerificationCache {
        StorageVerificationCache
    }

    async fn validate_input<'a, 'b>(
        &self,
        _interconnect: &dyn ModuleInterconect,
        _dbtx: &mut DatabaseTransaction<'b>,
        _verification_cache: &Self::VerificationCache,
        _input: &'a Self::Input,
    ) -> Result<InputMeta, ModuleError> {
        unimplemented!()
    }

    async fn apply_input<'a, 'b, 'c>(
        &'a self,
        _interconnect: &'a dyn ModuleInterconect,
        _dbtx: &mut DatabaseTransaction<'c>,
        _input: &'b Self::Input,
        _cache: &Self::VerificationCache,
    ) -> Result<InputMeta, ModuleError> {
        unimplemented!()
    }

    async fn validate_output(
        &self,
        _dbtx: &mut DatabaseTransaction,
        _output: &Self::Output,
    ) -> Result<TransactionItemAmount, ModuleError> {
        unimplemented!()
    }

    async fn apply_output<'a, 'b>(
        &'a self,
        _dbtx: &mut DatabaseTransaction<'b>,
        _output: &'a Self::Output,
        _out_point: OutPoint,
    ) -> Result<TransactionItemAmount, ModuleError> {
        unimplemented!()
    }

    async fn end_consensus_epoch<'a, 'b>(
        &'a self,
        _consensus_peers: &HashSet<PeerId>,
        _dbtx: &mut DatabaseTransaction<'b>,
    ) -> Vec<PeerId> {
        vec![]
    }

    async fn output_status(
        &self,
        _dbtx: &mut DatabaseTransaction<'_>,
        _out_point: OutPoint,
    ) -> Option<Self::OutputOutcome> {
        None
    }

    async fn audit(&self, _dbtx: &mut DatabaseTransaction<'_>, _audit: &mut Audit) {}

    fn api_endpoints(&self) -> Vec<ApiEndpoint<Self>> {
        vec![
            api_endpoint! {
            "/store",
            async |_module: &StorageModule, dbtx, param: u32| -> u32 {
                println!("Storing {}", param);
                let value = ExampleValue(param);
                dbtx.insert_entry(&ExampleKey(1), &value).await.expect("Could not insert entry");
                dbtx.commit_tx().await.expect("DB Error");
                Ok(param)
            }
            },
            api_endpoint! {
            "/retrieve",
            async |_module: &StorageModule, _dbtx, _request: ()| -> u32 {
                let value = _dbtx.get_value(&ExampleKey(1)).await.expect("Could not get entry");
                dbg!(&value);
                Ok(value.unwrap().0)
            }
            },
        ]
    }
}

impl StorageModule {
    /// Create new module instance
    pub fn new(cfg: StorageConfig) -> StorageModule {
        StorageModule { cfg }
    }
}

// Must be unique.
// TODO: we need to provide guidence for allocating these
pub const MODULE_KEY_STORAGE: u16 = 128;
plugin_types_trait_impl!(
    MODULE_KEY_STORAGE,
    StorageInput,
    StorageOutput,
    StorageOutputOutcome,
    StorageOutputConfirmation,
    StorageVerificationCache
);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Error)]
pub enum StorageError {
    #[error("Something went wrong")]
    SomethingDummyWentWrong,
}
