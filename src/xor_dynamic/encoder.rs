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

use rand::{rngs::ChaCha20Rng, seq::SliceRandom};
use std::collections::HashSet;
use thiserror::Error;

use crate::{
    core::encoder::{AsmInit, AsmInitWithSeed, Encoder},
    obfuscation::{
        aarch64::AArch64CodeAssembler,
        common::{AsmSaveRegisters, GarbageInstructions},
        x32::X32CodeAssembler,
        x64::X64CodeAssembler,
    },
    schema::encoder::SchemaDecoderStub,
};

pub type XorDynamicEncoderX64ChaCha = XorDynamicEncoder<X64CodeAssembler<ChaCha20Rng>>;

pub type XorDynamicEncoderX32ChaCha = XorDynamicEncoder<X32CodeAssembler<ChaCha20Rng>>;

pub type XorDynamicEncoderAArch64 = XorDynamicEncoder<AArch64CodeAssembler<ChaCha20Rng>>;

#[derive(Debug)]
pub struct XorDynamicEncoder<AsmType: XorDynamicStub> {
    pub encoding_count: u32,
    pub save_registers: bool,
    pub plain_decoder: bool,
    assembler: AsmType,
    stub_key_terminator: Vec<u8>,
    stub_payload_terminator: Vec<u8>,
    badchars: HashSet<u8>,
}

#[derive(Error, Debug)]
pub enum XorDynamicEncoderError {
    #[error("SchemaEncoderError")]
    SchemaEncoderError,
    #[error("BadCharacters")]
    BadCharacters,
    #[error("AssemblerError")]
    AssemblerError,
    #[error("NonExistentKey")]
    NonExistentKey,
    #[error("Key terminator could not be found for the xor dynamic encoder.")]
    NonExistentKeyTerminator,
    #[error("Payload terminator could not be found for the xor dynamic encoder.")]
    NonExistentPayloadTerminator,
    #[error("NoAvailableByteForStubTerminator.")]
    NoAvailableByteForStubTerminator,
}

pub struct XorDecoderStub {
    pub stub: Vec<u8>,
    pub key_terminator_stub: Vec<u8>,
    pub payload_terminator_stub: Vec<u8>,
}

pub trait XorDynamicStub {
    fn get_xor_dynamic_decoder_stub(&mut self, badchars: &HashSet<u8>) -> Result<XorDecoderStub, XorDynamicEncoderError>;
}

pub fn generate_key(
    buf: &[u8],
    badchars: &HashSet<u8>,
    key_chars: &[u8],
) -> Result<Vec<u8>, XorDynamicEncoderError> {
    let buf_len = buf.len();
    let min_len = {
        let pct = 0.2 + 0.05 * badchars.len() as f64;
        let val = (buf_len as f64 * pct / 100.0) as usize;
        val.max(1).min(buf_len)
    };

    let max_len = buf_len;
    let key_increment = {
        let pct = 0.01 + 0.001 * badchars.len() as f64;
        let val = (buf_len as f64 * pct / 100.0) as usize;
        val.max(1)
    };

    let mut key_len = min_len;

    while key_len <= max_len {
        let capped_key_len = key_len.min(max_len);
        let mut key = Vec::with_capacity(capped_key_len);

        for x in 0..capped_key_len {
            let valid_char = key_chars.iter().copied().find(|&candidate| {
                (0..)
                    .map(|i| i * capped_key_len + x)
                    .take_while(|&pos| pos < buf_len)
                    .all(|pos| !badchars.contains(&(buf[pos] ^ candidate)))
            });

            if let Some(c) = valid_char {
                key.push(c);
            } else {
                break;
            }
        }

        if key.len() == capped_key_len {
            return Ok(key);
        }

        key_len += key_increment;
    }

    Err(XorDynamicEncoderError::NonExistentKey)
}

impl<AsmType> XorDynamicEncoder<AsmType>
where
    AsmType: XorDynamicStub,
{
    pub fn builder() -> XorDynamicEncoderBuilder<AsmType> {
        XorDynamicEncoderBuilder::default()
    }
}

#[derive(Debug)]
pub struct XorDynamicEncoderBuilder<AsmType> {
    encoding_count: u32,
    save_registers: bool,
    plain_decoder: bool,
    _marker: std::marker::PhantomData<AsmType>,
}

impl<AsmType> Default for XorDynamicEncoderBuilder<AsmType> {
    fn default() -> Self {
        Self {
            encoding_count: 1,
            save_registers: false,
            plain_decoder: false,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<AsmType> XorDynamicEncoderBuilder<AsmType>
where
    AsmType: XorDynamicStub,
{
    pub fn set_encoding_count(mut self, count: u32) -> Self {
        self.encoding_count = count;
        self
    }

    pub fn set_plain_decoder(mut self, plain_decoder: bool) -> Self {
        self.plain_decoder = plain_decoder;
        self
    }

    pub fn set_save_registers(mut self, save: bool) -> Self {
        self.save_registers = save;
        self
    }

    pub fn build_with_rng_seed(self, seed: u64) -> XorDynamicEncoder<AsmType>
    where
        AsmType: AsmInitWithSeed,
    {
        let assembler = AsmType::new_with_rng(seed);
        let stub_key_terminator = vec![0x41];
        let stub_payload_terminator = vec![0x42, 0x42];
        let mut badchars: HashSet<u8> = HashSet::new();
        //badchars.insert(0x00);
        //badchars.insert(0x0a);
        //badchars.insert(0x0d);
        XorDynamicEncoder {
            encoding_count: self.encoding_count,
            save_registers: self.save_registers,
            plain_decoder: self.plain_decoder,
            assembler,
            stub_key_terminator,
            stub_payload_terminator,
            badchars,
        }
    }

    pub fn build(self) -> XorDynamicEncoder<AsmType>
    where
        AsmType: AsmInit,
    {
        let assembler = AsmType::new();
        let stub_key_terminator = vec![0x41]; // ASCII 'A' symbol, used as a terminator for the key in the stub
        let stub_payload_terminator = vec![0x42, 0x42]; // ASCII 'B, B' symbol, used as a terminator for the payload in the stub
        let mut badchars: HashSet<u8> = HashSet::new();
        //badchars.insert(0x00);
        //badchars.insert(0x0a);
        //badchars.insert(0x0d);

        XorDynamicEncoder {
            encoding_count: self.encoding_count,
            save_registers: self.save_registers,
            plain_decoder: self.plain_decoder,
            assembler,
            stub_key_terminator,
            stub_payload_terminator,
            badchars,
        }
    }
}

impl From<crate::schema::encoder::SchemaEncoderError> for XorDynamicEncoderError {
    fn from(_: crate::schema::encoder::SchemaEncoderError) -> Self {
        XorDynamicEncoderError::SchemaEncoderError
    }
}

impl<AsmType> Encoder for XorDynamicEncoder<AsmType>
where
    AsmType: XorDynamicStub + AsmSaveRegisters + GarbageInstructions + SchemaDecoderStub,
{
    fn encode(&mut self, payload: &[u8]) -> Result<Vec<u8>, Self::Error> {
        loop {
            let mut data = payload.to_vec();

            if self.save_registers {
                let save_registers_suffix = self.assembler.get_save_registers_suffix();
                data.extend_from_slice(&save_registers_suffix);
            }

            match self.encode_round(&data, self.encoding_count) {
                Ok(mut full_binary) => {
                    if self.save_registers {
                        let mut save_registers_prefix = self.assembler.get_save_registers_prefix();
                        save_registers_prefix.extend_from_slice(&full_binary);
                        full_binary = save_registers_prefix;
                    }

                    return Ok(full_binary);
                }
                Err(err) => {
                    if matches!(err, XorDynamicEncoderError::BadCharacters) {
                        continue;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
    }

    type Error = XorDynamicEncoderError;
}

impl<AsmType> XorDynamicEncoder<AsmType>
where
    AsmType: XorDynamicStub + GarbageInstructions + SchemaDecoderStub,
{
    fn encode_round(
        &mut self,
        payload: &[u8],
        iterations_remaining: u32,
    ) -> Result<Vec<u8>, XorDynamicEncoderError> {
        if iterations_remaining == 0 {
            return Ok(payload.to_vec());
        }

        let badchars = self.badchars.clone();
        let decoder_stub = self.assembler.get_xor_dynamic_decoder_stub(&badchars)?;
        let stub = decoder_stub.stub;
        let key_terminator_stub = decoder_stub.key_terminator_stub;
        let payload_terminator_stub = decoder_stub.payload_terminator_stub;
        let stub_without_terminators = stub
            .windows(key_terminator_stub.len())
            .filter(|w| *w != key_terminator_stub.as_slice())
            .collect::<Vec<_>>()
            .concat();

        let stub_cleaned = stub_without_terminators
            .windows(payload_terminator_stub.len())
            .filter(|w| *w != payload_terminator_stub.as_slice())
            .collect::<Vec<_>>()
            .concat();

        if has_badchars(&stub_cleaned, &badchars) {
            return Err(XorDynamicEncoderError::BadCharacters);
        }

        let key_chars: Vec<u8> = (1u8..=255).filter(|c| !badchars.contains(c)).collect();
        let key = generate_key(payload, &badchars, &key_chars)?;
        let key_terminator = generate_key_terminator(&key, &key_chars)?;

        let mut encoded: Vec<u8> = Vec::with_capacity(payload.len());

        for (pos, &b) in payload.iter().enumerate() {
            encoded.push(b ^ key[pos % key.len()]);
        }

        let payload_terminator = generate_payload_terminator(&encoded, &key_chars)?;

        let mut final_payload = Vec::new();

        let garbage = self.assembler.generate_garbage_instructions();
        final_payload.extend_from_slice(&garbage);

        let mut stub_replaced = stub.clone();
        stub_replaced =
            replace_subsequence(&stub_replaced, &key_terminator_stub, &[key_terminator]);
        stub_replaced = replace_subsequence(
            &stub_replaced,
            &payload_terminator_stub,
            &payload_terminator,
        );

        final_payload.extend_from_slice(&stub_replaced);
        final_payload.extend_from_slice(&key);
        final_payload.push(key_terminator);
        final_payload.extend_from_slice(&encoded);
        final_payload.extend_from_slice(&payload_terminator);

        let payload_length = (key.len() + 1 + encoded.len() + payload_terminator.len()) as u32;

        if !self.plain_decoder {
            let schema_size = (final_payload.len() - payload_length as usize) / 4 + 1;
            let random_schema = crate::schema::encoder::new_cipher_schema(schema_size);
            final_payload = crate::schema::encoder::schema_cipher(final_payload, &random_schema);
            final_payload = self
                .assembler
                .add_schema_decoder(final_payload, &random_schema)?;
        }

        if has_badchars(&final_payload, &badchars) {
            return Err(XorDynamicEncoderError::BadCharacters);
        }

        let full_binary = final_payload;
        self.encode_round(&full_binary, iterations_remaining - 1)
    }
}

fn generate_key_terminator(key: &[u8], key_chars: &[u8]) -> Result<u8, XorDynamicEncoderError> {
    let mut rng = rand::rng();
    let mut shuffled: Vec<u8> = key_chars.to_vec();
    shuffled.shuffle(&mut rng);

    shuffled
        .into_iter()
        .find(|&c| !key.contains(&c))
        .ok_or(XorDynamicEncoderError::NonExistentKeyTerminator)
}

fn generate_payload_terminator(
    encoded: &[u8],
    key_chars: &[u8],
) -> Result<Vec<u8>, XorDynamicEncoderError> {
    let mut rng = rand::rng();
    let mut pairs: Vec<(u8, u8)> = key_chars
        .iter()
        .flat_map(|&i| key_chars.iter().map(move |&j| (i, j)))
        .collect();

    pairs.shuffle(&mut rng);

    pairs
        .into_iter()
        .find_map(|(i, j)| {
            let pair = [i, j];
            if !find_subsequence(encoded, &pair) {
                Some(pair.to_vec())
            } else {
                None
            }
        })
        .ok_or(XorDynamicEncoderError::NonExistentPayloadTerminator)
}

fn has_badchars(buf: &[u8], badchars: &HashSet<u8>) -> bool {
    buf.iter().any(|b| badchars.contains(b))
}

fn replace_subsequence(haystack: &[u8], needle: &[u8], replacement: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i <= haystack.len() - needle.len() {
        if &haystack[i..i + needle.len()] == needle {
            result.extend_from_slice(replacement);
            i += needle.len();
        } else {
            result.push(haystack[i]);
            i += 1;
        }
    }

    result.extend_from_slice(&haystack[i..]);

    result
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}
