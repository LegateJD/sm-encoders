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

use dynasmrt::{
    dynasm, relocations::Relocation, aarch64::Aarch64Relocation, DynasmApi,
    DynasmLabelApi, VecAssembler,
};
use rand::{seq::IndexedRandom, Rng};
use crate::arm64::registers::{get_random_general_purpose_register, get_safe_random_general_purpose_register};

pub const SAFE_GARBAGE_INSTRUCTIONS: [fn(&mut VecAssembler<Aarch64Relocation>); 38] = [
    |assembler| {
        dynasm!(assembler
            ; .arch aarch64
            ; nop
        );
    },
    |assembler| {
        // msr nzcv, xzr
        assembler.extend(b"\x1f\x42\x1b\xd5");
    },
    |assembler| {

        // mrs x0, nzcv
        // eor x0, x0, #0x20000000
        // msr nzcv, x0

        assembler.extend(b"\x00\x42\x3b\xd5");
        assembler.extend(b"\x00\x00\x63\xd2");
        assembler.extend(b"\x00\x42\x1b\xd5");
    },
    |assembler| {
        dynasm!(assembler
            ; .arch aarch64
            ; yield
        );
    },
    |assembler| {
        dynasm!(assembler
            ; .arch aarch64
            ; cmp xzr, xzr
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; mov X(register_id), X(register_id)
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let second_register =
            get_safe_random_general_purpose_register(&[register.clone()]);
        let register_id = register.x as u32;
        let second_register_id = second_register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; mov X(second_register_id), X(register_id)
            ; mov X(register_id), X(second_register_id)
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; add XSP(register_id), XSP(register_id), 0
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; sub XSP(register_id), XSP(register_id), 0
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; and X(register_id), X(register_id), X(register_id)
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; orr X(register_id), X(register_id), X(register_id)
        );
    },
    /*|assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; eor XSP(register_id), X(register_id), 0
        );
    },*/
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; lsl X(register_id), X(register_id), #0
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; lsr X(register_id), X(register_id), 0
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; asr X(register_id), X(register_id), 0
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; ror X(register_id), X(register_id), 0
        );
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; .arch aarch64
            ; b >label
            ; =>label
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; cmp X(register_id), X(register_id)
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; tst X(register_id), X(register_id)
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), eq
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), ne
        );
    },

    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; cmn XSP(register_id), 0
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; mul X(register_id), X(register_id), X(register_id)
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;
        let mut rng = rand::rng();
        let random_byte = rng.random::<u8>();

        dynasm!(assembler
            ; .arch aarch64
            ; add XSP(register_id), XSP(register_id), random_byte as u32
            ; sub XSP(register_id), XSP(register_id), random_byte as u32
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; rev X(register_id), X(register_id)
            ; rev X(register_id), X(register_id)
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), hi
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), lo
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), cs
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), gt
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), lt
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), vs
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), mi
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), hs
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), ge
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), le
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), ls
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), cc
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), vc
        );
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; csel X(register_id), X(register_id), X(register_id), pl
        );
    }
];
