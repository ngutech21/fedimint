use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use base64::{engine::general_purpose, Engine as _};
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
    pub async fn store_data(&self, file: PathBuf) -> Result<String, StorageClientError> {
        let key = uuid::Uuid::new_v4().hyphenated().to_string();
        let content = self.read_file_as_base64(file);
        match self.context.api.store_data(key.clone(), content).await {
            Ok(_) => Ok(key),
            Err(e) => Err(StorageClientError::ApiError(e)),
        }
    }

    pub async fn store_data_raw(&self, raw_bytes: Vec<u8>) -> Result<String, StorageClientError> {
        let key = uuid::Uuid::new_v4().hyphenated().to_string();
        let file_content = general_purpose::STANDARD_NO_PAD.encode(raw_bytes);

        match self.context.api.store_data(key.clone(), file_content).await {
            Ok(_) => Ok(key),
            Err(e) => Err(StorageClientError::ApiError(e)),
        }
    }

    pub async fn retrieve_data(
        &self,
        key: String,
        file: PathBuf,
    ) -> Result<(), StorageClientError> {
        match self.context.api.retrieve_data(key).await {
            Ok(res) => {
                self.write_file_from_base64(res, file);
                Ok(())
            }

            Err(e) => Err(StorageClientError::ApiError(e)),
        }
    }

    pub async fn retrieve_data_raw(&self, key: String) -> Result<Vec<u8>, StorageClientError> {
        match self.context.api.retrieve_data(key).await {
            Ok(res) => Ok(self.decode_base64(res)),
            Err(e) => Err(StorageClientError::ApiError(e)),
        }
    }

    pub fn read_file_as_base64(&self, file_name: PathBuf) -> String {
        let file_content = fs::read(file_name).expect("Could not read file");
        general_purpose::STANDARD_NO_PAD.encode(file_content)
    }

    pub fn write_file_from_base64(&self, base64_content: String, file_name: PathBuf) {
        let file_content = general_purpose::STANDARD_NO_PAD
            .decode(base64_content)
            .expect("Could not decode base64");
        fs::write(file_name, file_content).expect("Could not write file");
    }

    pub fn decode_base64(&self, base64_content: String) -> Vec<u8> {
        general_purpose::STANDARD_NO_PAD
            .decode(base64_content)
            .expect("Could not decode base64")
    }
}
