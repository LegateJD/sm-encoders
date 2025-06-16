use rand::RngCore;

use crate::{
    asm::assembler::assemble,
    core::obfuscation::{CallOver, GarbageJump},
};

struct X64CodeAssembler {}

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
    fn add_call_over(payload: Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
        let call_assembly = format!("call 0x{:x}", payload.len() + 5);
        let mut final_bin = assemble(&call_assembly)?;
        final_bin.extend(payload.into_iter());

        Ok(final_bin)
    }
}
