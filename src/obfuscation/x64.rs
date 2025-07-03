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

use rand::RngCore;

use crate::{
    core::encoder::AsmInit, obfuscation::common::{CallOver, GarbageAssembly, GarbageInstructions, GarbageJump}, utils::randomization::coin_flip, x64_arch::garbage::generate_garbage_assembly
};

pub struct X64CodeAssembler {}

impl AsmInit for X64CodeAssembler {
    fn new() -> Self {
        X64CodeAssembler {  }
    }
}

impl GarbageJump for X64CodeAssembler {
    fn add_jmp_over(&self, payload: &[u8]) -> Vec<u8> {
        let len = payload.len() as i32 + 2;
        let mut bin = vec![0xE9u8];
        bin.extend(len.to_le_bytes());

        bin
    }

    fn generate_garbage_jump(&self) -> Vec<u8> {
        let mut rng = rand::rng();
        let mut random_bytes = [0; 10];
        rng.fill_bytes(&mut random_bytes);
        let mut final_bin = self.add_jmp_over(&random_bytes);
        final_bin.extend(random_bytes);

        final_bin
    }
}

impl CallOver for X64CodeAssembler {
    fn add_call_over(&self, payload: Vec<u8>) -> Vec<u8> {
        let len = payload.len() as i32 + 5;
        let mut bin = vec![0xE8u8];
        bin.extend(len.to_le_bytes());

        bin
    }
}

impl GarbageInstructions for X64CodeAssembler {
    fn generate_garbage_instructions(&self) -> Vec<u8> {
        let mut garbage_bin = self.generate_garbage_assembly();

        if coin_flip() {
            let mut jmp_garbage = self.generate_garbage_jump();

            if coin_flip() {
                garbage_bin.extend(jmp_garbage.into_iter());
            } else {
                jmp_garbage.extend(garbage_bin.into_iter());
                garbage_bin = jmp_garbage;
            }
        }

        garbage_bin
    }
}

impl GarbageAssembly for X64CodeAssembler {
    fn generate_garbage_assembly(&self) -> Vec<u8> {
        generate_garbage_assembly()
    }
}
