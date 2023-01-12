use fedimint_api::{
    core::client::ClientModule, module::TransactionItemAmount, Amount, ServerModule,
};
use fedimint_storage::{common::StorageModuleDecoder, config::StorageClientConfig, StorageModule};

#[derive(Debug)]
pub struct StorageClient {
    pub config: StorageClientConfig,
}

impl ClientModule for StorageClient {
    type Decoder = <StorageModule as ServerModule>::Decoder;
    type Module = StorageModule;
    const KIND: &'static str = "storage";

    fn decoder(&self) -> Self::Decoder {
        StorageModuleDecoder
    }

    fn input_amount(
        &self,
        _input: &<Self::Module as fedimint_api::ServerModule>::Input,
    ) -> fedimint_api::module::TransactionItemAmount {
        TransactionItemAmount {
            amount: Amount::ZERO,
            fee: Amount::ZERO,
        }
    }

    fn output_amount(
        &self,
        _output: &<Self::Module as fedimint_api::ServerModule>::Output,
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
