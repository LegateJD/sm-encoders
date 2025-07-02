use dynasmrt::{dynasm, x64::X64Relocation, DynasmApi, DynasmLabelApi, VecAssembler};

use crate::{obfuscation::x64::X64CodeAssembler, sgn::encoder::SgnDecoderStub, x64_arch::registers::{get_save_random_general_purpose_register, RCX_FULL}};

impl SgnDecoderStub for X64CodeAssembler {
    fn get_sgn_decoder_stub(
        &self,
        seed: u8,
        payload_size: usize,
    ) -> Result<Vec<u8>, anyhow::Error> {
        let mut assembler = VecAssembler::<X64Relocation>::new(0);
        let indexer_register = get_save_random_general_purpose_register(&[RCX_FULL]);
        let seed_register =
            get_save_random_general_purpose_register(&[RCX_FULL, indexer_register.clone()]);
        let indexer_register_id = indexer_register.quad as u8;
        let seed_register_id = seed_register.low as u8;

        dynasm!(assembler
            ; mov Rb(seed_register_id), seed as i8
            ; mov rcx, payload_size as i32
            ; lea Rq(indexer_register_id), [>data - 1]
            ; decode:
            ; xor BYTE [Rq(indexer_register_id) + rcx], Rb(seed_register_id)
            ; add Rb(seed_register_id), BYTE [Rq(indexer_register_id) + rcx]
            ; loop <decode
            ; data:
        );

        let bytes = assembler.finalize()?;

        Ok(bytes)
    }
}
