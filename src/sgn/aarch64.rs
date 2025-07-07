use dynasmrt::{aarch64::Aarch64Relocation, dynasm, x64::X64Relocation, x86::X86Relocation, DynasmApi, DynasmError, DynasmLabelApi, VecAssembler};

use crate::{obfuscation::{aarch64::AArch64CodeAssembler, x32::X32CodeAssembler}, sgn::encoder::{SgnDecoderStub, ShikataGaNaiError}, x64_arch::registers::{get_save_random_general_purpose_register, RCX_FULL}};

impl SgnDecoderStub for AArch64CodeAssembler {
    fn get_sgn_decoder_stub(
        &self,
        seed: u8,
        payload_size: usize,
    ) -> Result<Vec<u8>, ShikataGaNaiError> {
        let mut assembler = VecAssembler::<Aarch64Relocation>::new(0);
        let indexer_register = get_save_random_general_purpose_register(&[RCX_FULL]);
        let seed_register =
            get_save_random_general_purpose_register(&[RCX_FULL, indexer_register.clone()]);
        let indexer_register_id = indexer_register.quad as u8;
        let seed_register_id = seed_register.low as u8;

        dynasm!(assembler
            ; .arch aarch64
            ; mov w3, 0x42
            ; mov x2, 0x4242
            ; adr x1, >_data_sub1
            ; eor w4, w4, w3
            ; strb w4, [x1, x2]
            ; ldrb w5, [x1, x2] 
            ; add w3, w3, w5
            ; subs x2, x2, 1 
            ; b.ne >_decode
            ; _decode:
            ; ldrb w4, [x1, x2] 
            ; _data_sub1:
        );

        let bytes = assembler.finalize()?;

        Ok(bytes)
    }
}