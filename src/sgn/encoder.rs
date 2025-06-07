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

use crate::asm::assemble::assemble;
use rand::{seq::IndexedRandom, Rng};

#[derive(Debug)]
pub struct Encoder {
    seed: u8,
}

#[derive(Debug, Clone, Copy)]
struct Register {
    full: &'static str,
    extended: &'static str,
    high: &'static str,
    low: &'static str,
    arch: u8,
}

const GENERAL_PURPOSE_REGISTERS_64_BIT: &[Register] = &[
    Register {
        full: "RAX",
        extended: "EAX",
        high: "AX",
        low: "AL",
        arch: 64,
    },
    Register {
        full: "RBX",
        extended: "EBX",
        high: "BX",
        low: "BL",
        arch: 64,
    },
    Register {
        full: "RCX",
        extended: "ECX",
        high: "CX",
        low: "CL",
        arch: 64,
    },
    Register {
        full: "RDX",
        extended: "EDX",
        high: "DX",
        low: "DL",
        arch: 64,
    },
    Register {
        full: "RSI",
        extended: "ESI",
        high: "SI",
        low: "SIL",
        arch: 64,
    },
];

impl Encoder {
    pub fn new(seed: u8) -> Self {
        Encoder { seed }
    }

    pub fn encode(&self, mut payload: Vec<u8>) -> Vec<u8> {
        additive_feedback_loop(&mut payload, self.seed);
        let mut full_binary = self.get_decoder_stub(&payload).unwrap();
        full_binary.append(&mut payload);

        full_binary
    }

    fn get_decoder_assembly(&self, payload_size: usize) -> String {
        let decoder_template: String = "MOV {RL},{K}
	MOV RCX,{S}
	LEA {R},[RIP+data-1]
decode:
	XOR BYTE PTR [{R}+RCX],{RL}
	ADD {RL},BYTE PTR [{R}+RCX]
	LOOP decode
data:"
            .into();

        let reg1 = get_random_general_purpose_register(&["ECX"]);
        let reg2 = get_random_general_purpose_register(&["CL", reg1.full]);

        decoder_template
            .replace("{R}", &reg1.full)
            .replace("{RL}", &reg2.low)
            .replace("{K}", &self.seed.to_string())
            .replace("{S}", &payload_size.to_string())
    }

    fn get_decoder_stub(&self, payload: &[u8]) -> Result<Vec<u8>, u8> {
        let assembly = self.get_decoder_assembly(payload.len());
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

fn get_random_general_purpose_register(excludes: &[&str]) -> Register {
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

    let r = filtered
        .choose(&mut rng)
        .expect("Should always be able to choose a register from a non-empty slice");

    (**r).clone()
}
