use std::sync::Arc;

use fedimint_api::{
    core::client::ClientModule, module::TransactionItemAmount, Amount, ServerModule,
};
use fedimint_storage::{common::StorageModuleDecoder, config::StorageClientConfig, StorageModule};
use thiserror::Error;

use crate::{api::ApiError, utils::ClientContext};

#[derive(Debug)]
pub struct StorageClient {
    pub config: StorageClientConfig,
    pub context: Arc<ClientContext>,
}

#[derive(Debug, Error)]
pub enum StorageClientError {
    #[error("Error querying federation: {0}")]
    ApiError(#[from] ApiError),
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

    pub async fn store_data(&self, value: String) -> Result<String, StorageClientError> {
        // FIXME use result

        match self.context.api.store_data(value).await {
            Ok(res) => Ok(res),
            Err(e) => Err(StorageClientError::ApiError(e)),
        }

        // let _res = self
        //     .context
        //     .api
        //     .store_data(value)
        //     .await
        //     .map_err(|_e| StorageClientError::ApiError); // FIXME use result
        // Ok(())
        //println!(">>>> Storing data!");
    }

    pub async fn retrieve_data(&self, key: String) -> Result<String, StorageClientError> {
        println!(">>>> Retrieving data!");

        match self.context.api.retrieve_data(key).await {
            Ok(res) => Ok(res),
            Err(e) => Err(StorageClientError::ApiError(e)),
        }
    }
}
