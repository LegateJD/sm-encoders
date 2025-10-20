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

use std::{collections::HashSet};
use rand::seq::SliceRandom;
use thiserror::Error;

use crate::{
    core::encoder::{AsmInit, Encoder},
    obfuscation::{aarch64::AArch64CodeAssembler, x32::X32CodeAssembler, x64::X64CodeAssembler},
};

pub type XorDynamicEncoderX64 = XorDynamicEncoder<X64CodeAssembler>;

pub type XorDynamicEncoderX32 = XorDynamicEncoder<X32CodeAssembler>;

pub type XorDynamicEncoderAArch64 = XorDynamicEncoder<AArch64CodeAssembler>;

#[derive(Debug)]
pub struct XorDynamicEncoder<AsmType: XorDynamicStub> {
    assembler: AsmType,
    stub_key_terminator: Vec<u8>,
    stub_payload_terminator: Vec<u8>,
    badchars: HashSet<u8>,
}

#[derive(Error, Debug)]
pub enum XorDynamicEncoderError {
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
}

pub trait XorDynamicStub {
    fn get_decoder_stub(&self) -> Result<Vec<u8>, XorDynamicEncoderError>;
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
    AsmType: XorDynamicStub + AsmInit,
{
    pub fn new(seed: u8) -> Self {
        let assembler = AsmType::new();
        let stub_key_terminator = vec![0x41];
        let stub_payload_terminator = vec![0x42, 0x42];
        let mut badchars: HashSet<u8> = HashSet::new();
        badchars.insert(0x00);
        badchars.insert(0x0a);
        badchars.insert(0x0d);

        Self {
            assembler,
            stub_key_terminator,
            stub_payload_terminator,
            badchars,
        }
    }
}

impl<AsmType> Encoder for XorDynamicEncoder<AsmType>
where
    AsmType: XorDynamicStub + AsmInit,
{
    fn encode(&self, buf: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let badchars = self.badchars.clone();
        let stub = self.assembler.get_decoder_stub()?;

        let stub_without_terms = stub
            .windows(self.stub_key_terminator.len())
            .filter(|w| *w != self.stub_key_terminator.as_slice())
            .collect::<Vec<_>>()
            .concat();

        let stub_cleaned = stub_without_terms
            .windows(self.stub_payload_terminator.len())
            .filter(|w| *w != self.stub_payload_terminator.as_slice())
            .collect::<Vec<_>>()
            .concat();

        if has_badchars(&stub_cleaned, &badchars) {
            return Err(XorDynamicEncoderError::BadCharacters);
        }

        let key_chars: Vec<u8> = (1u8..=255).filter(|c| !badchars.contains(c)).collect();
        let key = generate_key(buf, &badchars, &key_chars)?;
        let key_term = generate_key_terminator(&key, &key_chars)?;

        let mut encoded: Vec<u8> = Vec::with_capacity(buf.len());

        for (pos, &b) in buf.iter().enumerate() {
            encoded.push(b ^ key[pos % key.len()]);
        }

        let payload_term = generate_payload_terminator(&encoded, &key_chars)?;

        let mut final_payload = Vec::new();

        let mut stub_replaced = stub.clone();
        stub_replaced = replace_subsequence(&stub_replaced, &self.stub_key_terminator, &[key_term]);
        stub_replaced = replace_subsequence(&stub_replaced, &self.stub_payload_terminator, &payload_term);
        final_payload.extend_from_slice(&stub_replaced);
        final_payload.extend_from_slice(&key);
        final_payload.push(key_term);
        final_payload.extend_from_slice(&encoded);
        final_payload.extend_from_slice(&payload_term);

        if has_badchars(&final_payload, &badchars) {
            return Err(XorDynamicEncoderError::BadCharacters);
        }

        Ok(final_payload)
    }

    type Error = XorDynamicEncoderError;
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

fn generate_payload_terminator(encoded: &[u8], key_chars: &[u8]) -> Result<Vec<u8>, XorDynamicEncoderError> {
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
