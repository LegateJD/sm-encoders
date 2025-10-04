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

use std::fmt;

use dynasmrt::{dynasm, x64::X64Relocation, DynasmApi, DynasmLabelApi, VecAssembler};

use byteorder::{BigEndian, ByteOrder, LittleEndian};
use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
};
use thiserror::Error;

use crate::{
    core::encoder::Encoder, obfuscation::{common::{CallOver, GarbageInstructions, GarbageJump}, x64::X64CodeAssembler}, sgn::encoder::SgnDecoderStub, x64_arch::registers::{get_save_random_general_purpose_register, RSP_FULL}
};
use crate::core::encoder::AsmInit;
use crate::obfuscation::aarch64::AArch64CodeAssembler;
use crate::obfuscation::x32::X32CodeAssembler;
use crate::sgn::encoder::SgnEncoder;

pub type SchemaEncoderX64 = SchemaEncoder<X64CodeAssembler>;

pub type SchemaEncoderX32 = SchemaEncoder<X32CodeAssembler>;

pub type SchemaEncoderAArch64 = SchemaEncoder<AArch64CodeAssembler>;

#[derive(Error, Debug)]
pub enum SchemaEncoderError {
    #[error("AssemblerError")]
    AssemblerError
}

pub struct SchemaEncoder<
    T: GarbageJump + CallOver + SgnDecoderStub + GarbageInstructions + SchemaDecoderStub,
> {
    assembler: T,
}

pub struct Operation {
    pub(crate) instruction: SchemaInstruction,
    pub(crate) key: Option<[u8; 4]>,
}

pub trait SchemaDecoderStub {
    fn add_schema_decoder(
        &self,
        payload: Vec<u8>,
        schema: &Vec<Operation>,
    ) -> Result<Vec<u8>, SchemaEncoderError>;
}

#[derive(Debug, Clone, Copy)]
pub enum SchemaInstruction {
    XOR,
    SUB,
    ADD,
    ROL,
    ROR,
    NOT,
}

impl<AsmType> SchemaEncoder<AsmType>
where
    AsmType: GarbageJump + CallOver + SgnDecoderStub + GarbageInstructions + SchemaDecoderStub + AsmInit
{
    pub fn new(seed: u8) -> Self {
        let assembler = AsmType::new();

        Self { assembler }
    }
}

impl fmt::Display for SchemaInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchemaInstruction::XOR => write!(f, "XOR"),
            SchemaInstruction::SUB => write!(f, "SUB"),
            SchemaInstruction::ADD => write!(f, "ADD"),
            SchemaInstruction::ROL => write!(f, "ROL"),
            SchemaInstruction::ROR => write!(f, "ROR"),
            SchemaInstruction::NOT => write!(f, "NOT"),
        }
    }
}

impl Distribution<SchemaInstruction> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SchemaInstruction {
        let index: u8 = rng.random_range(0..=5);
        match index {
            0 => SchemaInstruction::XOR,
            1 => SchemaInstruction::SUB,
            2 => SchemaInstruction::ADD,
            3 => SchemaInstruction::ROL,
            4 => SchemaInstruction::ROR,
            _ => SchemaInstruction::NOT,
        }
    }
}

impl<AsmType: GarbageJump + CallOver + SgnDecoderStub + GarbageInstructions + SchemaDecoderStub> Encoder
    for SchemaEncoder<AsmType>
{
    fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, Self::Error> {
        let mut bin = payload.to_vec();

        let mut garbage = self.assembler.generate_garbage_instructions();

        garbage.extend(bin.iter());

        println!("Hexadecimal representation:");
        for byte in &garbage {
            print!("{:02x} ", byte);
        }
        println!("\n");

        let schema_size = (garbage.len() - bin.len()) / 4 + 1;
        bin = garbage;

        let random_schema = new_cipher_schema(schema_size);
        bin = schema_cipher(bin, &random_schema);
        bin = self.assembler.add_schema_decoder(bin, &random_schema)?;

        Ok(bin)
    }
    
    type Error = SchemaEncoderError;
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
                let encoded = LittleEndian::read_u32(&payload[index..index + 4]).wrapping_sub(BigEndian::read_u32(&operation.key.unwrap()));
                LittleEndian::write_u32(&mut payload[index..index + 4], encoded)
            }
            SchemaInstruction::SUB => {
                let encoded = LittleEndian::read_u32(&payload[index..index + 4]).wrapping_add(BigEndian::read_u32(&operation.key.unwrap()));
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
