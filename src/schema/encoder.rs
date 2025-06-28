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

use crate::{
    core::obfuscation::{CallOver, Encode, GarbageInstructions, GarbageJump},
    sgn::encoder::SgnDecoderStub,
    x64_arch::{
        obfuscation::X64CodeAssembler,
        registers::{get_save_random_general_purpose_register, RSP_FULL},
    },
};

struct SchemaEncoder<
    T: GarbageJump + CallOver + SgnDecoderStub + GarbageInstructions + SchemaDecoder,
> {
    assembler: T,
}

struct Operation {
    instruction: SchemaInstruction,
    key: Option<[u8; 4]>,
}

pub trait SchemaDecoder {
    fn add_schema_decoder(
        &self,
        payload: Vec<u8>,
        schema: &Vec<Operation>,
    ) -> Result<Vec<u8>, anyhow::Error>;
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

impl<T: GarbageJump + CallOver + SgnDecoderStub + GarbageInstructions + SchemaDecoder> Encode
    for SchemaEncoder<T>
{
    fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
        let mut data = payload.to_vec();

        let mut garbage = self.assembler.generate_garbage_instructions()?;
        garbage.extend(data.iter());

        let schema_size = (garbage.len() - data.len()) / 8 + 1;
        data = garbage;

        let random_schema = new_cipher_schema(schema_size);
        data = schema_cipher(data, &random_schema);
        data = self.assembler.add_schema_decoder(data, &random_schema)?;

        Ok(data)
    }
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

impl SchemaDecoder for X64CodeAssembler {
    fn add_schema_decoder(
        &self,
        mut payload: Vec<u8>,
        schema: &Vec<Operation>,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let mut assembler = VecAssembler::<X64Relocation>::new(0);

        let mut garbage = self.generate_garbage_instructions()?;
        let mut index = garbage.len() as i32;
        garbage.extend(payload.into_iter());
        payload = garbage;

        payload = self.add_call_over(payload)?;
        garbage = self.generate_garbage_instructions()?;
        payload.extend(garbage.into_iter());

        let reg = get_save_random_general_purpose_register(&[RSP_FULL]);
        let indexer_register_id = reg.quad as u8;
        dynasm!(assembler
            ; pop Rq(indexer_register_id)
        );
        let pop = assembler.finalize()?;
        payload.extend(pop.into_iter());

        for operation in schema {
            garbage = self.generate_garbage_instructions()?;
            payload.extend(garbage.into_iter());
            assembler = VecAssembler::<X64Relocation>::new(0);

            match operation.key {
                Some(k) => {
                    match operation.instruction {
                        SchemaInstruction::XOR => {
                            dynasm!(assembler
                                ; xor DWORD [Rq(indexer_register_id) + index], BigEndian::read_u32(&k) as i32
                            );
                        }
                        SchemaInstruction::SUB => {
                            dynasm!(assembler
                                ; sub DWORD [Rq(indexer_register_id) + index], BigEndian::read_u32(&k) as i32
                            );
                        }
                        SchemaInstruction::ADD => {
                            dynasm!(assembler
                                ; add DWORD [Rq(indexer_register_id) + index], BigEndian::read_u32(&k) as i32
                            );
                        }
                        SchemaInstruction::ROL => {
                            dynasm!(assembler
                                ; rol DWORD [Rq(indexer_register_id) + index], BigEndian::read_u32(&k) as i8
                            );
                        }
                        SchemaInstruction::ROR => {
                            dynasm!(assembler
                                ; ror DWORD [Rq(indexer_register_id) + index], BigEndian::read_u32(&k) as i8
                            );
                        }
                        _ => unreachable!(),
                    }
                    dynasm!(assembler
                        ; xor DWORD [Rq(indexer_register_id) + index], BigEndian::read_u32(&k) as i32
                    );
                }
                None => {
                    dynasm!(assembler
                        ; not DWORD [Rq(indexer_register_id) + index]
                    );
                }
            };

            let decipher_step = assembler.finalize()?;
            payload.extend(decipher_step.into_iter());

            index += 4;
        }

        assembler = VecAssembler::<X64Relocation>::new(0);
        dynasm!(assembler
            ; jmp Rq(indexer_register_id)
        );

        let return_instruction = assembler.finalize()?;
        payload.extend(return_instruction.into_iter());

        Ok(payload)
    }
}
