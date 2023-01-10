use fedimint_api::{
    core::{client::ClientModulePlugin, ModuleKey, MODULE_KEY_STORAGE},
    module::TransactionItemAmount,
    Amount, ServerModulePlugin,
};
use fedimint_storage::{common::StorageModuleDecoder, config::StorageClientConfig, StorageModule};

#[derive(Debug)]
pub struct StorageClient {
    pub config: StorageClientConfig,
}

impl ClientModulePlugin for StorageClient {
    type Decoder = <StorageModule as ServerModulePlugin>::Decoder;
    type Module = StorageModule;
    const MODULE_KEY: ModuleKey = MODULE_KEY_STORAGE;

    fn decoder(&self) -> &'static Self::Decoder {
        &StorageModuleDecoder
    }
    fn input_amount(
        &self,
        _input: &<Self::Module as fedimint_api::ServerModulePlugin>::Input,
    ) -> fedimint_api::module::TransactionItemAmount {
        TransactionItemAmount {
            amount: Amount::ZERO,
            fee: Amount::ZERO,
        }
    }

    fn output_amount(
        &self,
        _output: &<Self::Module as fedimint_api::ServerModulePlugin>::Output,
    ) -> fedimint_api::module::TransactionItemAmount {
        TransactionItemAmount {
            amount: Amount::ZERO,
            fee: Amount::ZERO,
        }
    }
}

impl StorageClient {
    pub fn say_hello(&self) {
        println!(">>>> Hello from StorageClient!");
    }
}
