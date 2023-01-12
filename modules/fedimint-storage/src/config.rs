use fedimint_api::config::{
    ClientModuleConfig, TypedClientModuleConfig, TypedServerModuleConfig,
    TypedServerModuleConsensusConfig,
};
use fedimint_api::core::ModuleKind;
use fedimint_api::module::__reexports::serde_json;
use fedimint_api::PeerId;
use serde::{Deserialize, Serialize};

use crate::KIND;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Contains all configuration that will be encrypted such as private key material
    pub private: StorageConfigPrivate,
    /// Contains all configuration that needs to be the same for every federation member
    pub consensus: StorageConfigConsensus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageConfigConsensus {
    pub something: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageConfigPrivate {
    pub something_private: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StorageClientConfig {
    pub something: u64,
}

impl TypedClientModuleConfig for StorageClientConfig {
    fn kind(&self) -> fedimint_api::core::ModuleKind {
        KIND
    }
}

// impl TypedServerModuleConsensusConfig for StorageConfigConsensus {
//     fn to_client_config(&self) -> ClientModuleConfig {
//         serde_json::to_value(&StorageClientConfig {
//             something: self.something,
//         })
//         .expect("Serialization can't fail")
//         .into()
//     }
// }

impl TypedServerModuleConsensusConfig for StorageConfigConsensus {
    fn to_client_config(&self) -> ClientModuleConfig {
        ClientModuleConfig::new(
            KIND,
            serde_json::to_value(&StorageClientConfig {
                something: self.something,
            })
            .expect("Serialization can't fail"),
        )
    }
}

impl TypedServerModuleConfig for StorageConfig {
    type Local = ();
    type Private = StorageConfigPrivate;
    type Consensus = StorageConfigConsensus;

    fn from_parts(_local: Self::Local, private: Self::Private, consensus: Self::Consensus) -> Self {
        Self { private, consensus }
    }

    fn to_parts(self) -> (ModuleKind, Self::Local, Self::Private, Self::Consensus) {
        (KIND, (), self.private, self.consensus)
    }

    fn validate_config(&self, _identity: &PeerId) -> anyhow::Result<()> {
        Ok(())
    }
}
