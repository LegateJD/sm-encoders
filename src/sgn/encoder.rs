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

use crate::{
    asm::assembler::assemble,
    sgn::{
        obfuscate::generate_garbage_instructions,
        x64_architecture::{Register, GENERAL_PURPOSE_REGISTERS_64_BIT},
    },
};
use keystone_engine::KeystoneError;
use rand::seq::IndexedRandom;
use thiserror::Error;

#[derive(Debug)]
pub struct SgnEncoder {
    seed: u8,
    plain_decoder: bool,
}

#[derive(Error, Debug)]
pub enum SgnError {
    #[error("Assembler Engine failed.")]
    AssemblerError(#[from] KeystoneError),
}

impl SgnEncoder {
    pub fn new(seed: u8, plain_decoder: bool) -> Self {
        SgnEncoder {
            seed,
            plain_decoder,
        }
    }

    pub fn encode(&self, mut payload: Vec<u8>) -> Result<Vec<u8>, SgnError> {
        additive_feedback_loop(&mut payload, self.seed);
        let mut full_binary = self.generate_decoder_stub(&payload)?;
        full_binary.append(&mut payload);

        if self.plain_decoder {
            return Ok(full_binary);
        }

        let mut garbage = generate_garbage_instructions()?;

        garbage.extend(full_binary.into_iter());

        Ok(garbage)
    }

    fn generate_decoder_assembly(&self, payload_size: usize) -> String {
        let decoder_template: String = "MOV {RL},{K}
	MOV RCX,{S}
	LEA {R},[RIP+data-1]
decode:
	XOR BYTE PTR [{R}+RCX],{RL}
	ADD {RL},BYTE PTR [{R}+RCX]
	LOOP decode
data:"
            .into();

        let register1 = get_save_random_general_purpose_register(&["ECX"]);
        let register2 = get_save_random_general_purpose_register(&["CL", register1.full]);

        decoder_template
            .replace("{R}", &register1.full)
            .replace("{RL}", &register2.low)
            .replace("{K}", &self.seed.to_string())
            .replace("{S}", &payload_size.to_string())
    }

    fn generate_decoder_stub(&self, payload: &[u8]) -> Result<Vec<u8>, SgnError> {
        let assembly = self.generate_decoder_assembly(payload.len());
        assemble(&assembly)
    }
}

fn additive_feedback_loop(payload: &mut Vec<u8>, mut seed: u8) {
    for byte in payload.iter_mut().rev() {
        let original = *byte;
        *byte ^= seed;
        seed = original.wrapping_add(seed);
    }
}

pub fn get_save_random_general_purpose_register(excludes: &[&str]) -> &'static Register {
    let mut rng = rand::rng();
    let mut filtered = vec![];

    for reg in GENERAL_PURPOSE_REGISTERS_64_BIT.iter() {
        if !excludes.contains(&reg.extended)
            && !excludes.contains(&reg.full)
            && !excludes.contains(&reg.high)
            && !excludes.contains(&reg.low)
        {
            filtered.push(reg);
        }
    }

    let register = filtered.choose(&mut rng).unwrap();

    *register
}

pub fn get_random_general_purpose_register() -> &'static Register {
    let mut rng = rand::rng();
    let register = GENERAL_PURPOSE_REGISTERS_64_BIT.choose(&mut rng).unwrap();

    register
}
