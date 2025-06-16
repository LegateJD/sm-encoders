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
    asm::assembler::assemble, core::obfuscation::{CallOver, GarbageJump}, sgn::{
        instructions::SchemaInstruction,
        obfuscate::generate_garbage_instructions,
        x64_architecture::{AsmRegister, RCX_Full, Register, GENERAL_PURPOSE_REGISTERS_64_BIT},
    }
};
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
use iced_x86::{
    code_asm::{ah, al, asm_traits::CodeAsmMovsb, ax, byte_ptr, rcx, AsmRegister64, AsmRegister8, CodeAssembler},
    Register,
};
use keystone_engine::KeystoneError;
use rand::{seq::IndexedRandom, Rng};
use thiserror::Error;

#[derive(Debug)]
pub struct SgnEncoder<T: GarbageJump + CallOver> {
    assembler: T,
    seed: u8,
    plain_decoder: bool,
}

struct Operation {
    instruction: SchemaInstruction,
    key: Option<[u8; 4]>,
}

pub trait DecoderStub {
    fn get_decoder_st(&self, payload: &[u8]) -> Result<Vec<u8>, anyhow::Error>;
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

    fn generate_decoder_stub(&self, payload: &[u8]) -> Result<Vec<u8>, SgnError> {
        let payload_size = payload.len();
        let indexer_register = get_save_random_general_purpose_register(&[RCX_Full]);
        let seed_register =
            get_save_random_general_purpose_register(&[RCX_Full, indexer_register.clone()]);

        let mut a = CodeAssembler::new(64).unwrap();
        let mut data = a.create_label();
        let mut decode = a.create_label();

        let seed_register_low: AsmRegister8 = seed_register.low.into();
        let indexer_register_full: AsmRegister64 = indexer_register.full.into();
        a.mov(seed_register_low, self.seed as u32);
        a.mov(rcx, payload_size.into());
        a.lea(indexer_register_full, byte_ptr(data)).unwrap();
        a.set_label(&mut decode);
        a.xor(
            byte_ptr(indexer_register_full - 1 + rcx),
            seed_register_low,
        )
        .unwrap();

        a.add(
            seed_register_low,
            byte_ptr(indexer_register_full + rcx),
        )
        .unwrap();
        a.loop_(decode);
        a.set_label(&mut data);

        let bytes = a.assemble(0).unwrap();

        Ok(bytes)
    }
}

fn additive_feedback_loop(payload: &mut [u8], mut seed: u8) {
    for byte in payload.iter_mut().rev() {
        let original = *byte;
        *byte ^= seed;
        seed = original.wrapping_add(seed);
    }
}

pub fn get_save_random_general_purpose_register(excludes: &[AsmRegister]) -> &'static AsmRegister {
    let mut rng = rand::rng();
    let mut filtered = vec![];

    for reg in GENERAL_PURPOSE_REGISTERS_64_BIT.iter() {
        if !excludes.contains(&reg)
            && !excludes.contains(&reg)
            && !excludes.contains(&reg)
            && !excludes.contains(&reg)
        {
            filtered.push(reg);
        }
    }

    let register = filtered.choose(&mut rng).unwrap();

    *register
}

pub fn get_random_general_purpose_register() -> &'static AsmRegister {
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
