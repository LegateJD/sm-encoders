use rand::{seq::IndexedRandom, Rng, RngCore};

use crate::{asm::assemble::assemble, sgn::{
    encoder::get_random_general_purpose_register,
    instructions::{CONDITIONAL_JUMP_MNEMONICS, SAFE_GARBAGE_INSTRUCTIONS},
    utils::{coin_flip, random_label},
}};

fn generate_garbage_instructions() -> Result<Vec<u8>, String>{
    let garbage_assembly = generate_garbage_assembly();
    assemble(&garbage_assembly).map_err(|x| "dsdsd".into())
}

fn generate_garbage_assembly() -> String {
    if coin_flip() {
        return ";".to_string();
    }

    let garbage_assembly = get_random_safe_assembly();
    let register = get_random_general_purpose_register();
    let mut rng = rand::rng();
    let random_byte = format!("0x{:#04x}", rng.random::<u8>());
    garbage_assembly
        .replace("{R}", &register.full)
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

fn generate_garbage_jump() -> Vec<u8> {
    let mut rng = rand::rng();
    let mut random_bytes = vec![0; 10];
    rng.fill_bytes(&mut random_bytes);

    random_bytes
}

fn add_jmp_over(payload: Vec<u8>) -> Vec<u8> {
    let jmp_assembly = format!("jmp 0x{:#04x}", payload.len() + 2);
    let mut final_bin = assemble(&jmp_assembly).map_err(|x| "dsdsd".to_string()).unwrap();
    final_bin.extend(payload.into_iter());

    final_bin
}
