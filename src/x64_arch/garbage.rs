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

use dynasmrt::{dynasm, x64::X64Relocation, DynasmApi, DynasmLabelApi, VecAssembler};
use rand::{seq::IndexedRandom, Rng};

use crate::{utils::randomization::coin_flip, x64_arch::registers::get_random_general_purpose_register};

pub const SAFE_GARBAGE_INSTRUCTIONS: [fn(&mut VecAssembler<X64Relocation>) -> i32; 66] = [
    |assembler| {
        dynasm!(assembler
            ; nop
        );

        return 1;
    },
    |assembler| {
        dynasm!(assembler
            ; cld
        );

        return 1;
    },
    |assembler| {
        dynasm!(assembler
            ; clc
        );

        return 1;
    },
    |assembler| {
        dynasm!(assembler
            ; cmc
        );

        return 1;
    },
    |assembler| {
        dynasm!(assembler
            ; pause
        );

        return 1;
    },
    |assembler| {
        dynasm!(assembler
            ; fnop
        );

        return 1;
    },
    |assembler| {
        dynasm!(assembler
            ; fxam
        );

        return 1;
    },
    |assembler| {
        dynasm!(assembler
            ; ftst
        );

        return 1;
    },
    |assembler| {
        dynasm!(assembler
            ; jmp 2
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; rol Rq(register_id), 0
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; ror Rq(register_id), 0
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; shl Rq(register_id), 0
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; shr Rq(register_id), 0
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; rcl Rq(register_id), 0
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; rcr Rq(register_id), 0
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; sal Rq(register_id), 0
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; sar Rq(register_id), 0
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; xor Rq(register_id), 0
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; sub Rq(register_id), 0
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; add Rq(register_id), 0
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; and Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; or Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; bt Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmp Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; mov Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; xchg Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; test Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmova Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovb Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovc Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmove Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovg Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovl Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovo Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovp Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovs Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovz Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovae Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovge Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovle Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovna Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnb Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnc Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovne Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovng Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnl Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovno Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnp Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovns Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnz Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovpe Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovpo Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovbe Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnae Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnbe Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnle Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnge Rq(register_id), Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jmp =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; not Rq(register_id)
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; not Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; neg Rq(register_id)
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; neg Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; inc Rq(register_id)
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; inc Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; dec Rq(register_id)
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; dec Rq(register_id)
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;
        let mut rng = rand::rng();
        let random_byte = rng.random::<u8>();

        dynasm!(assembler
            ; add Rq(register_id), random_byte as i32
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; sub Rq(register_id), random_byte as i32
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;
        let mut rng = rand::rng();
        let random_byte = rng.random::<u8>();

        dynasm!(assembler
            ; sub Rq(register_id), random_byte as i32
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; add Rq(register_id), random_byte as i32
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;
        let mut rng = rand::rng();
        let random_byte = rng.random::<u8>();

        dynasm!(assembler
            ; ror Rq(register_id), random_byte as i8
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; rol Rq(register_id), random_byte as i8
        );

        return 1;
    },
    |assembler| {
        let register = get_random_general_purpose_register();
        let register_id = register.quad as u8;
        let mut rng = rand::rng();
        let random_byte = rng.random::<u8>();

        dynasm!(assembler
            ; rol Rq(register_id), random_byte as i8
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; ror Rq(register_id), random_byte as i8
        );

        return 1;
    },
];

pub const CONDITIONAL_JUMP_MNEMONICS: [fn(&mut VecAssembler<X64Relocation>) -> i32; 30] = [
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jae =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; ja =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jbe =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jb =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jc =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; je =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jge =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jg =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jle =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jl =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnae =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jna =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnbe =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnb =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnc =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jne =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnge =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jng =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnle =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnl =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jno =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnp =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jns =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnz =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jo =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jpe =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jpo =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jp =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; js =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
    |assembler| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jz =>label
        );

        get_random_safe_assembly(assembler);

        dynasm!(assembler
            ; =>label
        );

        return 1;
    },
];

pub fn generate_garbage_assembly() -> Vec<u8> {
    let mut assembler = VecAssembler::<X64Relocation>::new(0);
    get_random_safe_assembly(&mut assembler);
    let result = assembler.finalize().unwrap();

    result
}

fn get_random_safe_assembly(assembler: &mut VecAssembler<X64Relocation>) {
    if coin_flip() {
        return;
    }

    let mut rng = rand::rng();

    if coin_flip() {
        let add_garbage = SAFE_GARBAGE_INSTRUCTIONS.choose(&mut rng).unwrap();
        add_garbage(assembler);
    } else {
        let add_garbage_jump = SAFE_GARBAGE_INSTRUCTIONS.choose(&mut rng).unwrap();
        add_garbage_jump(assembler);
    }
}

