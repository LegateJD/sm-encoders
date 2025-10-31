use thiserror::Error;

use crate::{
    core::encoder::{AsmInit, Encoder},
    obfuscation::{
        x64::X64CodeAssembler
    },
};
use crate::obfuscation::aarch64::AArch64CodeAssembler;
use crate::obfuscation::common::{AsmSaveRegisters, GarbageInstructions};
use crate::obfuscation::x32::X32CodeAssembler;
use crate::schema::encoder::SchemaDecoderStub;

#[derive(Error, Debug)]
pub enum ShikataGaNaiError {
    #[error("AssemblerError")]
    AssemblerError,
    #[error("Schema encoder error")]
    SchemaEncoder
}

pub type SgnEncoderX64 = SgnEncoder<X64CodeAssembler>;

pub type SgnEncoderX32 = SgnEncoder<X32CodeAssembler>;

pub type SgnEncoderAArch64 = SgnEncoder<AArch64CodeAssembler>;

#[derive(Debug)]
pub struct SgnEncoder<AsmType: SgnDecoderStub> {
    seed: u8,
    assembler: AsmType,
    plain_decoder: bool,
    encoding_count: u32,
    save_registers: bool,
}

pub trait SgnDecoderStub {
    fn get_sgn_decoder_stub(&self, seed: u8, payload_size: usize)
        -> Result<Vec<u8>, ShikataGaNaiError>;
}

impl<AsmType> SgnEncoder<AsmType>
where
    AsmType: SgnDecoderStub + AsmInit
{
    pub fn new(seed: u8, plain_decoder: bool, encoding_count: u32, save_registers: bool) -> Self {
        let assembler = AsmType::new();

        Self { seed, assembler, plain_decoder, encoding_count, save_registers }
    }
}

impl From<crate::schema::encoder::SchemaEncoderError> for ShikataGaNaiError {
    fn from(_: crate::schema::encoder::SchemaEncoderError) -> Self {
        ShikataGaNaiError::SchemaEncoder
    }
}

impl<AsmType> Encoder for SgnEncoder<AsmType>
where
    AsmType: SgnDecoderStub + AsmInit + SchemaDecoderStub + GarbageInstructions + AsmSaveRegisters
{
    type Error = ShikataGaNaiError;

    fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let mut data = payload.to_vec();

        if self.save_registers {
            let save_registers_suffix = self.assembler.get_save_registers_suffix();
            data.extend(save_registers_suffix.iter());
        }

        let mut full_binary = self.encode_recursive(&data, self.encoding_count)?;

        if self.save_registers {
            let mut save_registers_prefix = self.assembler.get_save_registers_prefix();
            save_registers_prefix.extend(full_binary.iter());
            full_binary = save_registers_prefix;
        }

        Ok(full_binary)
    }
}

impl<AsmType> SgnEncoder<AsmType>
where
    AsmType: SgnDecoderStub + AsmInit + SchemaDecoderStub + GarbageInstructions + AsmSaveRegisters
{
    fn encode_recursive(&self, payload: &[u8], iterations_remaining: u32) -> Result<Vec<u8>, ShikataGaNaiError> {
        if iterations_remaining == 0 {
            return Ok(payload.to_vec());
        }

        let mut data = payload.to_vec();
        let mut garbage = self.assembler.generate_garbage_instructions();
        garbage.extend(data.iter());
        data = garbage;
        additive_feedback_loop(&mut data, self.seed);
        let mut full_binary = self.assembler.get_sgn_decoder_stub(self.seed, data.len())?;
        full_binary.extend(data.iter());

        if !self.plain_decoder {
            let schema_size = (full_binary.len() - data.len()) / 4 + 1;
            let random_schema = crate::schema::encoder::new_cipher_schema(schema_size);
            full_binary = crate::schema::encoder::schema_cipher(full_binary, &random_schema);
            full_binary = self.assembler.add_schema_decoder(full_binary, &random_schema)?;
        }

        self.encode_recursive(&full_binary, iterations_remaining - 1)
    }
}

fn additive_feedback_loop(payload: &mut [u8], mut seed: u8) {
    for byte in payload.iter_mut().rev() {
        let original = *byte;
        *byte ^= seed;
        seed = original.wrapping_add(seed);
    }
}
