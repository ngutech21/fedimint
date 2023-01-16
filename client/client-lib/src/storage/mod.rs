use std::sync::Arc;

use fedimint_api::{
    core::client::ClientModule, module::TransactionItemAmount, Amount, ServerModule,
};
use fedimint_storage::{common::StorageModuleDecoder, config::StorageClientConfig, StorageModule};
use thiserror::Error;

use crate::utils::ClientContext;

#[derive(Debug)]
pub struct StorageClient {
    pub config: StorageClientConfig,
    pub context: Arc<ClientContext>,
}

#[derive(Debug, Error)]
pub enum StorageClientError {
    #[error("ApiError")]
    ApiError,
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

    pub async fn store_data(&self, value: u32) -> Result<(), StorageClientError> {
        // FIXME use result
        let _res = self
            .context
            .api
            .store_data(value)
            .await
            .map_err(|_e| StorageClientError::ApiError); // FIXME use result
        Ok(())
        //println!(">>>> Storing data!");

        // let ws_api = WsFederationApi::from_config(client.config().as_ref());
        //     let response: Value = ws_api
        //         .request(
        //             &method,
        //             arg,
        //             EventuallyConsistent::new(ws_api.peers().len()),
        //         )
        //         .await
        //         .unwrap();

        //     Ok(CliOutput::UntypedApiOutput { value: response })
    }

    pub fn retrieve_data(&self) {
        println!(">>>> Retrieving data!");
    }
}
