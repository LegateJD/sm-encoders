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

use rand::{seq::IndexedRandom, Rng, RngCore};

use crate::{
    asm::assembler::assemble, core::utils::{coin_flip, random_label}, sgn::instructions::{CONDITIONAL_JUMP_MNEMONICS, SAFE_GARBAGE_INSTRUCTIONS}, x64_arch::registers::get_random_general_purpose_register
};

pub fn generate_garbage_instructions() -> Result<Vec<u8>, anyhow::Error> {
    let garbage_assembly = generate_garbage_assembly();
    let mut garbage_bin = assemble(&garbage_assembly)?;

    if coin_flip() {
        let mut jmp_garbage = generate_garbage_jump()?;

        if coin_flip() {
            garbage_bin.extend(jmp_garbage.into_iter());
        } else {
            jmp_garbage.extend(garbage_bin.into_iter());
            garbage_bin = jmp_garbage;
        }
    }

    Ok(garbage_bin)
}

fn generate_garbage_assembly() -> String {
    if coin_flip() {
        return ";".to_string();
    }

    let garbage_assembly = get_random_safe_assembly();
    let register = get_random_general_purpose_register();
    let mut rng = rand::rng();
    let random_byte = format!("0x{:x}", rng.random::<u8>());
    garbage_assembly
        .replace("{R}", &register.quad.to_string())
        .replace("{K}", &random_byte)
        .replace("{L}", &random_label(5))
        .replace("{G}", &generate_garbage_assembly())
}

fn get_random_safe_assembly() -> String {
    let mut rng = rand::rng();

    if coin_flip() {
        return SAFE_GARBAGE_INSTRUCTIONS
            .choose(&mut rng)
            .unwrap()
            .to_string();
    } else {
        return CONDITIONAL_JUMP_MNEMONICS
            .choose(&mut rng)
            .unwrap()
            .to_string()
            + " {L};{G};{L}:";
    }
}

fn generate_garbage_jump() -> Result<Vec<u8>, anyhow::Error> {
    let mut rng = rand::rng();
    let mut random_bytes = vec![0; 10];
    rng.fill_bytes(&mut random_bytes);
    let garbage_jmp = add_jmp_over(random_bytes)?;

    Ok(garbage_jmp)
}

fn add_jmp_over(payload: Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let jmp_assembly = format!("jmp 0x{:x}", payload.len() + 2);
    let mut final_bin = assemble(&jmp_assembly)?;
    final_bin.extend(payload.into_iter());

    Ok(final_bin)
}
