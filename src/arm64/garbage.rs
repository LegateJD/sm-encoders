use dynasmrt::{
    dynasm, relocations::Relocation, aarch64::Aarch64Relocation, DynasmApi,
    DynasmLabelApi, VecAssembler,
};
use rand::{seq::IndexedRandom, Rng};
use crate::arm64::registers::get_random_general_purpose_register;

pub const SAFE_GARBAGE_INSTRUCTIONS: [fn(&mut VecAssembler<Aarch64Relocation>); 18] = [
    |assembler| {
        dynasm!(assembler
            ; nop
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
];
