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
        instructions::{SchemaInstruction},
        obfuscate::generate_garbage_instructions,
        x64_architecture::{Register, GENERAL_PURPOSE_REGISTERS_64_BIT},
    },
};
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use keystone_engine::KeystoneError;
use rand::{seq::IndexedRandom, Rng};
use thiserror::Error;

#[derive(Debug)]
pub struct SgnEncoder {
    seed: u8,
    plain_decoder: bool,
}

struct Operation {
    instruction: SchemaInstruction,
    key: Option<[u8; 4]>,
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

    pub fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, SgnError> {
        let mut data = payload.to_vec();
        additive_feedback_loop(&mut data, self.seed);
        let mut full_binary = self.generate_decoder_stub(&payload)?;
        full_binary.extend(data.iter());

        if self.plain_decoder {
            return Ok(full_binary);
        }

        let mut garbage = generate_garbage_instructions()?;
        garbage.extend(full_binary.iter());

        let schema_size = (garbage.len() - full_binary.len()) / 8 + 1;
        full_binary = garbage;

        let random_schema = new_cipher_schema(schema_size);
        full_binary = schema_cipher(full_binary, &random_schema);
        full_binary = add_schema_decoder(full_binary, &random_schema)?;

        Ok(full_binary)
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

        let indexer_register = get_save_random_general_purpose_register(&["ECX"]);
        let seed_register = get_save_random_general_purpose_register(&["CL", indexer_register.full]);

        decoder_template
            .replace("{R}", &indexer_register.full)
            .replace("{RL}", &seed_register.low)
            .replace("{K}", &self.seed.to_string())
            .replace("{S}", &payload_size.to_string())
    }

    fn generate_decoder_stub(&self, payload: &[u8]) -> Result<Vec<u8>, SgnError> {
        let assembly = self.generate_decoder_assembly(payload.len());
        assemble(&assembly)
    }
}

fn additive_feedback_loop(payload: &mut [u8], mut seed: u8) {
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

fn new_cipher_schema(size: usize) -> Vec<Operation> {
    let mut schema = Vec::with_capacity(size);
    let mut rng = rand::rng();

    for _ in 0..size {
        let instruction: SchemaInstruction = rng.random();

        let key = match instruction {
            SchemaInstruction::NOT => None,
            SchemaInstruction::ROL | SchemaInstruction::ROR => Some([0u8, 0u8, 0u8, rng.random()]),
            _ => Some([rng.random(), rng.random(), rng.random(), rng.random()]),
        };

        let operation = Operation { instruction, key };
        schema.push(operation);
    }

    schema
}

fn add_schema_decoder(mut payload: Vec<u8>, schema: &Vec<Operation>) -> Result<Vec<u8>, SgnError> {
    let mut garbage = generate_garbage_instructions()?;
    let mut index = garbage.len();
    garbage.extend(payload.into_iter());
    payload = garbage;

    payload = add_call_over(payload)?;
    garbage = generate_garbage_instructions()?;
    payload.extend(garbage.into_iter());

    let reg = get_save_random_general_purpose_register(&["RSP"]);
    let pop_assembly = format!("POP {};", &reg.full);
    let pop = assemble(&pop_assembly)?;
    payload.extend(pop.into_iter());

    for operation in schema {
        garbage = generate_garbage_instructions()?;
        payload.extend(garbage.into_iter());

        let step_assembly = match operation.key {
            Some(k) => format!(
                "\t{} DWORD PTR [{}+0x{:x}],0x{:x};\n",
                operation.instruction,
                reg.full,
                index,
                BigEndian::read_u32(&k)
            ),
            None => format!(
                "\t{} DWORD PTR [{}+0x{:x}];\n",
                operation.instruction, reg.full, index
            ),
        };

        let decipher_step = assemble(&step_assembly)?;

        payload.extend(decipher_step.into_iter());

        index += 4;
    }

    let return_instruction = assemble(&format!("jmp {};", reg.full))?;
    payload.extend(return_instruction.into_iter());

    Ok(payload)
}

fn add_call_over(payload: Vec<u8>) -> Result<Vec<u8>, SgnError> {
    let call_assembly = format!("call 0x{:x}", payload.len() + 5);
    let mut final_bin = assemble(&call_assembly)?;
    final_bin.extend(payload.into_iter());

    Ok(final_bin)
}

fn schema_cipher(mut payload: Vec<u8>, schema: &Vec<Operation>) -> Vec<u8> {
    let mut index = 0;
    for operation in schema {
        match operation.instruction {
            SchemaInstruction::XOR => {
                let encoded = BigEndian::read_u32(&payload[index..index + 4])
                    ^ LittleEndian::read_u32(&operation.key.unwrap());
                BigEndian::write_u32(&mut payload[index..index + 4], encoded)
            }
            SchemaInstruction::ADD => {
                let encoded = (LittleEndian::read_u32(&payload[index..index + 4])
                    - BigEndian::read_u32(&operation.key.unwrap()))
                    % 0xFFFFFFFF;
                LittleEndian::write_u32(&mut payload[index..index + 4], encoded)
            }
            SchemaInstruction::SUB => {
                let encoded = (LittleEndian::read_u32(&payload[index..index + 4])
                    + BigEndian::read_u32(&operation.key.unwrap()))
                    % 0xFFFFFFFF;
                LittleEndian::write_u32(&mut payload[index..index + 4], encoded)
            }
            SchemaInstruction::ROL => {
                let encoded = LittleEndian::read_u32(&payload[index..index + 4])
                    .rotate_right(BigEndian::read_u32(&operation.key.unwrap()));
                LittleEndian::write_u32(&mut payload[index..index + 4], encoded)
            }
            SchemaInstruction::ROR => {
                let encoded = LittleEndian::read_u32(&payload[index..index + 4])
                    .rotate_left(BigEndian::read_u32(&operation.key.unwrap()));
                LittleEndian::write_u32(&mut payload[index..index + 4], encoded)
            }
            SchemaInstruction::NOT => {
                let encoded = !BigEndian::read_u32(&payload[index..index + 4]);
                BigEndian::write_u32(&mut payload[index..index + 4], encoded)
            }
        }

        index += 4;
    }

    payload
}
