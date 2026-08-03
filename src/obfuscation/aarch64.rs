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

use rand::{Rng, rngs::ThreadRng};

use crate::{core::encoder::AsmInit, obfuscation::common::{CallOver, GarbageAssembly, GarbageInstructions, GarbageJump}, utils::rng::RngCoinFlip, x64_arch::garbage::generate_garbage_x64_assembly};

pub struct AArch64CodeAssembler<RngType: Rng> {
    pub rng: RngType
}

impl AsmInit for AArch64CodeAssembler<ThreadRng> {
    fn new() -> Self {
        let rng = rand::rng();
        AArch64CodeAssembler { rng: rng }
    }
}

impl<RngType: Rng> GarbageJump for AArch64CodeAssembler<RngType> {
    fn add_jmp_over(&self, payload: &[u8]) -> Vec<u8> {
        let words = (payload.len() + 3) / 4 + 1;
        let imm26 = (words & 0x3FFFFFF) as u32;
        let instruction = 0x14000000 | imm26;
        let mut bin = Vec::with_capacity(4);
        bin.extend_from_slice(&instruction.to_le_bytes());
        bin
    }

    fn generate_garbage_jump(&mut self) -> Vec<u8> {
        let mut random_bytes = [0; 12];
        self.rng.fill_bytes(&mut random_bytes);
        let mut final_bin = self.add_jmp_over(&random_bytes);
        final_bin.extend(random_bytes);

        final_bin
    }
}

impl<RngType: Rng> CallOver for AArch64CodeAssembler<RngType> {
    fn add_call_over(&self, payload: Vec<u8>) -> Vec<u8> {
        let words = ((payload.len() + 3) / 4) + 2;

        let imm24 = (words & 0x00FFFFFF) as u32;
        let instruction = 0xEB000000 | imm24;

        let mut bin = Vec::with_capacity(4);
        bin.extend_from_slice(&instruction.to_le_bytes());

        bin
    }
}

impl<RngType: Rng> GarbageInstructions for AArch64CodeAssembler<RngType> {
    fn generate_garbage_instructions(&mut self) -> Vec<u8> {
        let mut garbage_bin = self.generate_garbage_assembly();

        if self.rng.coin_flip() {
            let mut jmp_garbage = self.generate_garbage_jump();

            if self.rng.coin_flip() {
                garbage_bin.extend(jmp_garbage.into_iter());
            } else {
                jmp_garbage.extend(garbage_bin.into_iter());
                garbage_bin = jmp_garbage;
            }
        }

        garbage_bin
    }
}

impl<RngType: Rng> GarbageAssembly for AArch64CodeAssembler<RngType> {
    fn generate_garbage_assembly(&mut self) -> Vec<u8> {
        generate_garbage_x64_assembly(&mut self.rng)
    }
}