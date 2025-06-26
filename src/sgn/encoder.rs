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

use thiserror::Error;

use crate::core::obfuscation::{CallOver, GarbageJump};

#[derive(Debug)]
pub struct SgnEncoder<T: GarbageJump + CallOver + SgnDecoderStub> {
    seed: u8,
    assembler: T
}

pub trait SgnDecoderStub {
    fn get_sgn_decoder_stub(&self, seed: u8, payload_size: usize) -> Result<Vec<u8>, anyhow::Error>;
}

#[derive(Error, Debug)]
pub enum SgnError {
    #[error("Assembler Engine failed.")]
    AssemblerError,
}

impl<T: GarbageJump + CallOver + SgnDecoderStub> SgnEncoder<T> {
    pub fn new(seed: u8, assembler: T) -> Self {
        SgnEncoder {
            seed,
            assembler
        }
    }

    pub fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
        let mut data = payload.to_vec();
        additive_feedback_loop(&mut data, self.seed);

        let mut full_binary = self.assembler.get_sgn_decoder_stub(self.seed, data.len())?;
        full_binary.extend(data.iter());

        Ok(full_binary)
    }
}

fn additive_feedback_loop(payload: &mut [u8], mut seed: u8) {
    for byte in payload.iter_mut().rev() {
        let original = *byte;
        *byte ^= seed;
        seed = original.wrapping_add(seed);
    }
}
