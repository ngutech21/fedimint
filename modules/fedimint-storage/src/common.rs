use std::io;

use fedimint_api::core::PluginDecode;
use fedimint_api::encoding::{Decodable, DecodeError};
use fedimint_api::module::registry::ModuleDecoderRegistry;

use crate::{StorageInput, StorageOutput, StorageOutputConfirmation, StorageOutputOutcome};

#[derive(Debug, Default, Clone)]
pub struct StorageModuleDecoder;

impl PluginDecode for StorageModuleDecoder {
    type Input = StorageInput;
    type Output = StorageOutput;
    type OutputOutcome = StorageOutputOutcome;
    type ConsensusItem = StorageOutputConfirmation;

    fn decode_input(&self, mut d: &mut dyn io::Read) -> Result<StorageInput, DecodeError> {
        StorageInput::consensus_decode(&mut d, &ModuleDecoderRegistry::default())
    }

    fn decode_output(&self, mut d: &mut dyn io::Read) -> Result<StorageOutput, DecodeError> {
        StorageOutput::consensus_decode(&mut d, &ModuleDecoderRegistry::default())
    }

    fn decode_output_outcome(
        &self,
        mut d: &mut dyn io::Read,
    ) -> Result<StorageOutputOutcome, DecodeError> {
        StorageOutputOutcome::consensus_decode(&mut d, &ModuleDecoderRegistry::default())
    }

    fn decode_consensus_item(
        &self,
        mut r: &mut dyn io::Read,
    ) -> Result<StorageOutputConfirmation, DecodeError> {
        StorageOutputConfirmation::consensus_decode(&mut r, &ModuleDecoderRegistry::default())
    }
}
