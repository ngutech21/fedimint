use std::io;

use fedimint_api::core::{ConsensusItem, Input, Output, OutputOutcome, PluginDecode};
use fedimint_api::encoding::{Decodable, DecodeError};
use fedimint_api::module::registry::ModuleDecoderRegistry;

use crate::{StorageInput, StorageOutput, StorageOutputConfirmation, StorageOutputOutcome};

#[derive(Debug, Default, Clone)]
pub struct StorageModuleDecoder;

impl PluginDecode for StorageModuleDecoder {
    fn decode_input(mut d: &mut dyn io::Read) -> Result<Input, DecodeError> {
        Ok(Input::from(StorageInput::consensus_decode(
            &mut d,
            &ModuleDecoderRegistry::default(),
        )?))
    }
    fn decode_output(mut d: &mut dyn io::Read) -> Result<Output, DecodeError> {
        Ok(Output::from(StorageOutput::consensus_decode(
            &mut d,
            &ModuleDecoderRegistry::default(),
        )?))
    }

    fn decode_output_outcome(mut d: &mut dyn io::Read) -> Result<OutputOutcome, DecodeError> {
        Ok(OutputOutcome::from(StorageOutputOutcome::consensus_decode(
            &mut d,
            &ModuleDecoderRegistry::default(),
        )?))
    }

    fn decode_consensus_item(
        mut r: &mut dyn io::Read,
    ) -> Result<fedimint_api::core::ConsensusItem, DecodeError> {
        Ok(ConsensusItem::from(
            StorageOutputConfirmation::consensus_decode(&mut r, &ModuleDecoderRegistry::default())?,
        ))
    }
}
