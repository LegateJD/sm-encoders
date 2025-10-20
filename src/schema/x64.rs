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

use byteorder::{BigEndian, ByteOrder};
use dynasmrt::{dynasm, x64::X64Relocation, DynasmApi, DynasmError, DynasmLabelApi, VecAssembler};
use crate::{obfuscation::{common::{CallOver, GarbageInstructions}, x64::X64CodeAssembler}, schema::encoder::{Operation, SchemaDecoderStub, SchemaEncoderError, SchemaInstruction}, x64_arch::registers::{get_save_random_general_purpose_register, RSP_FULL}};

impl SchemaDecoderStub for X64CodeAssembler {
    fn add_schema_decoder(
        &self,
        mut payload: Vec<u8>,
        schema: &Vec<Operation>,
    ) -> Result<Vec<u8>, SchemaEncoderError> {
        let mut assembler = VecAssembler::<X64Relocation>::new(0);

        let mut garbage = self.generate_garbage_instructions();
        let mut index = garbage.len() as i32;
        garbage.extend(payload.into_iter());
        payload = garbage;

        payload = self.add_call_over(payload);
        garbage = self.generate_garbage_instructions();
        payload.extend(garbage.into_iter());

        let reg = get_save_random_general_purpose_register(&[RSP_FULL]);
        let indexer_register_id = reg.quad as u8;
        dynasm!(assembler
            ; pop Rq(indexer_register_id)
        );
        let pop = assembler.finalize()?;
        payload.extend(pop.into_iter());

        for operation in schema {
            garbage = self.generate_garbage_instructions();
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

impl From<DynasmError> for SchemaEncoderError {
    fn from(value: DynasmError) -> Self {
        SchemaEncoderError::AssemblerError
    }
}
