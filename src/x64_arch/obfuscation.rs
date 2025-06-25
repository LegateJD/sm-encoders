use dynasmrt::{dynasm, x64::X64Relocation, DynasmApi, DynasmLabelApi, VecAssembler};

use rand::RngCore;

use crate::{
    asm::assembler::assemble,
    core::{
        obfuscation::{CallOver, GarbageAssembly, GarbageInstructions, GarbageJump},
        utils::coin_flip,
    },
    sgn::encoder::SgnDecoderStub,
    x64_arch::registers::{get_save_random_general_purpose_register, RCX_FULL},
};

pub struct X64CodeAssembler {}

impl GarbageJump for X64CodeAssembler {
    fn get_jmp_over(&self, payload: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
        let len = payload.len() + 2;
        let jmp_assembly = format!("jmp 0x{:x}", len);
        let mut final_bin = assemble(&jmp_assembly)?;
        final_bin.extend(payload.into_iter());

        Ok(final_bin)
    }

    fn generate_garbage_jump(&self) -> Result<Vec<u8>, anyhow::Error> {
        let mut rng = rand::rng();
        let mut random_bytes = [0; 10];
        rng.fill_bytes(&mut random_bytes);
        let mut final_bin = self.get_jmp_over(&random_bytes)?;
        final_bin.extend(random_bytes);

        Ok(final_bin)
    }
}

impl CallOver for X64CodeAssembler {
    fn add_call_over(&self, payload: Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
        let call_assembly = format!("call 0x{:x}", payload.len() + 5);
        let mut final_bin = assemble(&call_assembly)?;
        final_bin.extend(payload.into_iter());

        Ok(final_bin)
    }
}

impl GarbageInstructions for X64CodeAssembler {
    fn generate_garbage_instructions(&self) -> Result<Vec<u8>, anyhow::Error> {
        let garbage_assembly = self.generate_garbage_assembly();
        let mut garbage_bin = assemble(&garbage_assembly)?;

        if coin_flip() {
            let mut jmp_garbage = self.generate_garbage_jump()?;

            if coin_flip() {
                garbage_bin.extend(jmp_garbage.into_iter());
            } else {
                jmp_garbage.extend(garbage_bin.into_iter());
                garbage_bin = jmp_garbage;
            }
        }

        Ok(garbage_bin)
    }
}

impl GarbageAssembly for X64CodeAssembler {
    fn generate_garbage_assembly() -> Vec<u8> {
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
}

impl SgnDecoderStub for X64CodeAssembler {
    fn get_sgn_decoder_stub(
        &self,
        seed: i8,
        payload_size: usize,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let mut assembler = VecAssembler::<X64Relocation>::new(0);
        let indexer_register = get_save_random_general_purpose_register(&[RCX_FULL]);
        let seed_register =
            get_save_random_general_purpose_register(&[RCX_FULL, indexer_register.clone()]);
        let indexer_register_id = indexer_register.quad as u8;
        let seed_register_id = seed_register.low as u8;

        dynasm!(assembler
            ; mov Rb(seed_register_id), seed
            ; mov rcx, payload_size as i32
            ; lea Rq(indexer_register_id), [>data - 1]
            ; xor BYTE [Rq(indexer_register_id) + rcx], Rb(seed_register_id)
            ; add Rb(seed_register_id), BYTE [Rq(indexer_register_id) + rcx]
            ; loop <decode
            ; data:
        );

        let bytes = assembler.finalize()?;

        Ok(bytes)
    }
}
