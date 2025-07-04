use dynasmrt::{aarch64::Aarch64Relocation, dynasm, x64::X64Relocation, x86::X86Relocation, DynasmApi, DynasmError, DynasmLabelApi, VecAssembler};

use crate::{obfuscation::aarch64::AArch64CodeAssembler, xor_dynamic::encoder::{XorDynamicEncoderError, XorDynamicStub}};

impl XorDynamicStub for AArch64CodeAssembler {
    fn get_decoder_stub(&self) -> Result<Vec<u8>, XorDynamicEncoderError> {
        let mut assembler = VecAssembler::<Aarch64Relocation>::new(0);

        dynasm!(assembler
            ; .arch aarch64
            ; b >_call
            ; _ret:
            ; ldr x19, [sp], #8
            ; sub sp, sp, #8
            ; str x19, [sp]
            ; ldr x20, [sp], #8
            ; mov w0, 'A' as u32
            ; _lp1:
            ; ldrb w1, [x20]
            ; cmp w1, w0
            ; b.ne <_lp1
            ; sub sp, sp, #8
            ; str x20, [sp]
            ; ldr x21, [sp], #8
            ; _lp2:
            ; sub sp, sp, #8
            ; str x19, [sp]
            ; ldr x22, [sp], #8
            ; _lp3:
            ; ldrb w0, [x22]
            ; ldrb w1, [x20]
            ; eor w1, w1, w0
            ; strb w1, [x20]
            ; add x20, x20, #1
            ; add x22, x22, #1
            ; ldrh w2, [x20]
            ; mov w3, 0x4242
            ; cmp w2, w3
            ; b.eq >_jmp
            ; ldrb w4, [x22]
            ; cmp w4,'A' as u32
            ; b.ne <_lp3
            ; b <_lp2
            ; _jmp:
            ; br x21
            ; _call:
            ; bl <_ret
        );

        let bytes = assembler.finalize()?;

        Ok(bytes)
    }
}
