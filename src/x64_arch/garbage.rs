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
    dynasm, relocations::Relocation, x64::X64Relocation, x86::X86Relocation, DynasmApi,
    DynasmLabelApi, VecAssembler,
};
use rand::{Rng, RngExt, rngs::ThreadRng, seq::IndexedRandom};

use crate::{
    utils::{rng::RngCoinFlip}, x64_arch::registers::get_random_general_purpose_register,
};

trait SuperRng: Rng + RngCoinFlip {}


pub const SAFE_GARBAGE_INSTRUCTIONS: [fn(&mut VecAssembler<X64Relocation>, &mut dyn Rng); 66] = [
    |assembler, rng| {
        dynasm!(assembler
            ; nop
        );
    },
    |assembler, rng| {

        dynasm!(assembler
            ; cld
        );
    },
    |assembler, rng| {
        dynasm!(assembler
            ; clc
        );
    },
    |assembler, rng| {
        dynasm!(assembler
            ; cmc
        );
    },
    |assembler, rng| {
        dynasm!(assembler
            ; pause
        );
    },
    |assembler, rng| {
        dynasm!(assembler
            ; fnop
        );
    },
    |assembler, rng| {
        dynasm!(assembler
            ; fxam
        );
    },
    |assembler, rng| {
        dynasm!(assembler
            ; ftst
        );
    },
    |assembler, rng| {
        dynasm!(assembler
            ; jmp 2
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; rol Rq(register_id), 0
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; ror Rq(register_id), 0
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; shl Rq(register_id), 0
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; shr Rq(register_id), 0
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; rcl Rq(register_id), 0
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; rcr Rq(register_id), 0
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; sal Rq(register_id), 0
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; sar Rq(register_id), 0
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; xor Rq(register_id), 0
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; sub Rq(register_id), 0
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; add Rq(register_id), 0
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; and Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; or Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; bt Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmp Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; mov Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; xchg Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; test Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmova Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovb Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovc Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmove Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovg Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovl Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovo Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovp Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovs Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovz Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovae Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovge Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovle Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovna Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnb Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnc Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovne Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovng Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnl Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovno Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnp Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovns Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnz Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovpe Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovpo Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovbe Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnae Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnbe Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnle Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; cmovnge Rq(register_id), Rq(register_id)
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jmp =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; not Rq(register_id)
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; not Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; neg Rq(register_id)
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; neg Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; inc Rq(register_id)
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; inc Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;

        dynasm!(assembler
            ; dec Rq(register_id)
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; dec Rq(register_id)
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;
        let random_byte = rng.random::<u8>();

        dynasm!(assembler
            ; add Rq(register_id), random_byte as i32
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; sub Rq(register_id), random_byte as i32
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;
        let random_byte = rng.random::<u8>();

        dynasm!(assembler
            ; sub Rq(register_id), random_byte as i32
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; add Rq(register_id), random_byte as i32
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;
        let random_byte = rng.random::<u8>();

        dynasm!(assembler
            ; ror Rq(register_id), random_byte as i8
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; rol Rq(register_id), random_byte as i8
        );
    },
    |assembler, rng| {
        let register = get_random_general_purpose_register(rng);
        let register_id = register.quad as u8;
        let random_byte = rng.random::<u8>();

        dynasm!(assembler
            ; rol Rq(register_id), random_byte as i8
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; ror Rq(register_id), random_byte as i8
        );
    },
];

pub const CONDITIONAL_JUMP_MNEMONICS: [fn(&mut VecAssembler<X64Relocation>, &mut dyn Rng); 30] = [
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jae =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; ja =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jbe =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jb =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jc =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; je =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jge =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jg =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jle =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jl =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnae =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jna =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnbe =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnb =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnc =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jne =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnge =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jng =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnle =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnl =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jno =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnp =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jns =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jnz =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jo =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jpe =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jpo =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jp =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; js =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
    |assembler, rng| {
        let label = assembler.new_dynamic_label();

        dynasm!(assembler
            ; jz =>label
        );

        get_random_safe_assembly(assembler, rng);

        dynasm!(assembler
            ; =>label
        );
    },
];

pub fn generate_garbage_x64_assembly<T: Rng>(rng: &mut T) -> Vec<u8> {
    let mut assembler = VecAssembler::<X64Relocation>::new(0);
    get_random_safe_assembly(&mut assembler, rng);
    let result = assembler.finalize().unwrap();

    result
}

pub fn generate_garbage_x32_assembly<T: Rng>(rng: &mut T) -> Vec<u8> {
    let mut assembler = VecAssembler::<X64Relocation>::new(0);
    get_random_safe_assembly(&mut assembler, rng);
    let result = assembler.finalize().unwrap();

    result
}

fn get_random_safe_assembly(assembler: &mut VecAssembler<X64Relocation>, rng: &mut dyn Rng) {
    if rng.coin_flip() {
        return;
    }

    if rng.coin_flip() {
        let add_garbage = SAFE_GARBAGE_INSTRUCTIONS.choose(rng).unwrap();
        add_garbage(assembler, rng);
    } else {
        let add_garbage_jump = CONDITIONAL_JUMP_MNEMONICS.choose(rng).unwrap();
        add_garbage_jump(assembler, rng);
    }
}
