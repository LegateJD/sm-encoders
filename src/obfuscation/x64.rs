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

use rand::{SeedableRng, rand_core::RngCore, rngs::{ChaCha12Rng, ThreadRng}};

use crate::{
    core::encoder::{AsmInit, AsmInitWithRng}, obfuscation::common::{CallOver, GarbageAssembly, GarbageInstructions, GarbageJump}, utils::rng::RngCoinFlip, x64_arch::garbage::generate_garbage_x64_assembly
};
use crate::obfuscation::common::AsmSaveRegisters;

pub struct X64CodeAssembler<RngType: RngCore> {
    pub rng: RngType
}

impl AsmInit for X64CodeAssembler<ThreadRng> {
    fn new() -> Self {
        let rng = rand::rng();
        X64CodeAssembler { rng: rng }
    }
}

impl AsmInit for X64CodeAssembler<ChaCha12Rng> {
    fn new() -> Self {
        let rng = ChaCha12Rng::seed_from_u64(98573457);
        X64CodeAssembler { rng: rng }
    }
}

impl<RngType: RngCore> AsmInitWithRng<RngType> for X64CodeAssembler<RngType> {
    fn new_with_rng(rng: RngType) -> Self {
        X64CodeAssembler { rng: rng  }
    }
}

impl<RngType: RngCore> GarbageJump for X64CodeAssembler<RngType> {
    fn add_jmp_over(&self, payload: &[u8]) -> Vec<u8> {
        let len = payload.len() as i32;
        let mut bin = vec![0xE9u8];
        bin.extend(len.to_le_bytes());

        bin
    }

    fn generate_garbage_jump(&mut self) -> Vec<u8> {
        let mut random_bytes = [0; 10];
        self.rng.fill_bytes(&mut random_bytes);
        let mut final_bin = self.add_jmp_over(&random_bytes);
        final_bin.extend(random_bytes);

        final_bin
    }
}

impl<RngType: RngCore> CallOver for X64CodeAssembler<RngType> {
    fn add_call_over(&self, payload: Vec<u8>) -> Vec<u8> {
        let len = payload.len() as i32;
        let mut bin = vec![0xE8u8];
        bin.extend(len.to_le_bytes());
        bin.extend(payload);

        bin
    }
}

impl<RngType: RngCore> GarbageInstructions for X64CodeAssembler<RngType> {
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

impl<RngType: RngCore> GarbageAssembly for X64CodeAssembler<RngType> {
    fn generate_garbage_assembly(&mut self) -> Vec<u8> {
        generate_garbage_x64_assembly(&mut self.rng)
    }
}

impl<RngType: RngCore> AsmSaveRegisters for X64CodeAssembler<RngType> {
    fn get_save_registers_suffix(&self) -> Vec<u8> {
        vec![0x41, 0x5f, 0x41, 0x5e, // POP R15,R14
             0x41, 0x5d, 0x41, 0x5c, // POP R13,R12
             0x41, 0x5b, 0x41, 0x5a, // POP R11,R10
             0x41, 0x59, 0x41, 0x58, // POP R9,R8
             0x5c, 0x5d, 0x5f, 0x5e, // POP RSP,RBP,RDI,RSI
             0x5a, 0x59, 0x5b, 0x58] // POP RDX,RCX,RBX,RAX
    }

    fn get_save_registers_prefix(&self) -> Vec<u8> {
        vec![0x41, 0x5f, 0x41, 0x5e, // POP R15,R14
             0x41, 0x5d, 0x41, 0x5c, // POP R13,R12
             0x41, 0x5b, 0x41, 0x5a, // POP R11,R10
             0x41, 0x59, 0x41, 0x58, // POP R9,R8
             0x5c, 0x5d, 0x5f, 0x5e, // POP RSP,RBP,RDI,RSI
             0x5a, 0x59, 0x5b, 0x58] // POP RDX,RCX,RBX,RAX
    }
}
