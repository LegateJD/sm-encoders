/*
 * Copyright 2025 Mykyta Zakharov
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use rand::rngs::{ChaCha12Rng, ChaCha20Rng, ThreadRng};
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

pub type SgnEncoderX64 = SgnEncoder<X64CodeAssembler<ChaCha12Rng>>;

pub type SgnEncoderX32 = SgnEncoder<X32CodeAssembler<ChaCha12Rng>>;

pub type SgnEncoderAArch64 = SgnEncoder<AArch64CodeAssembler<ChaCha12Rng>>;

pub type SgnEncoderX64ChaCha = SgnEncoder<X64CodeAssembler<ChaCha20Rng>>;
pub type SgnEncoderX32ChaCha = SgnEncoder<X32CodeAssembler<ChaCha20Rng>>;
pub type SgnEncoderAArch64ChaCha = SgnEncoder<AArch64CodeAssembler<ChaCha20Rng>>;

#[derive(Debug)]
pub struct SgnEncoder<AsmType: SgnDecoderStub> {
    seed: u8,
    assembler: AsmType,
    plain_decoder: bool,
    encoding_count: u32,
    save_registers: bool,
}

pub trait SgnDecoderStub {
    fn get_sgn_decoder_stub(&mut self, seed: u8, payload_size: usize)
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

    pub fn builder() -> SgnEncoderBuilder<AsmType> {
        SgnEncoderBuilder::default()
    }
}

#[derive(Debug)]
pub struct SgnEncoderBuilder<AsmType> {
    seed: u8,
    plain_decoder: bool,
    encoding_count: u32,
    save_registers: bool,
    _marker: std::marker::PhantomData<AsmType>,
}

impl<AsmType> Default for SgnEncoderBuilder<AsmType> {
    fn default() -> Self {
        Self { seed: 0, plain_decoder: false, encoding_count: 1, save_registers: false, _marker: std::marker::PhantomData }
    }
}

impl<AsmType> SgnEncoderBuilder<AsmType>
where
    AsmType: SgnDecoderStub + AsmInit,
{
    pub fn set_seed(mut self, seed: u8) -> Self {
        self.seed = seed;
        self
    }

    pub fn set_plain_decoder(mut self, plain: bool) -> Self {
        self.plain_decoder = plain;
        self
    }

    pub fn set_encoding_count(mut self, count: u32) -> Self {
        self.encoding_count = count;
        self
    }

    pub fn set_save_registers(mut self, save: bool) -> Self {
        self.save_registers = save;
        self
    }

    pub fn build(self) -> SgnEncoder<AsmType> {
        let assembler = AsmType::new();
        SgnEncoder { seed: self.seed, assembler, plain_decoder: self.plain_decoder, encoding_count: self.encoding_count, save_registers: self.save_registers }
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

    fn encode(&mut self, payload: &[u8]) -> Result<Vec<u8>, Self::Error> {
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
    fn encode_recursive(&mut self, payload: &[u8], iterations_remaining: u32) -> Result<Vec<u8>, ShikataGaNaiError> {
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
