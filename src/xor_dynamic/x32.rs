use dynasmrt::{dynasm, x64::X64Relocation, x86::X86Relocation, DynasmApi, DynasmError, DynasmLabelApi, VecAssembler};

use crate::{obfuscation::{x32::X32CodeAssembler, x64::X64CodeAssembler}, xor_dynamic::encoder::{XorDynamicEncoderError, XorDynamicStub}};


impl XorDynamicStub for X32CodeAssembler {
    fn get_decoder_stub(&self) -> Result<Vec<u8>, XorDynamicEncoderError> {
        let mut assembler = VecAssembler::<X86Relocation>::new(0);
        dynasm!(assembler
            ; .arch x86
            ; jmp >_call
            ; _ret:
            ; pop ebx
            ; mov edi, ebx
            ; mov al, 'A' as i8
            ; cld
            ; _lp1:
            ; scasb
            ; jne <_lp1
            ; mov ecx, edi
            ; _lp2:
            ; mov esi, ebx
            ; _lp3:
            ; mov al, BYTE [esi]
            ; xor BYTE [edi], al
            ; inc edi
            ; cmp WORD [edi], 0x4242
            ; je >_jmp
            ; inc esi
            ; cmp BYTE [esi], 'A' as i8
            ; jne <_lp3
            ; jmp <_lp2
            ; _jmp:
            ; jmp ecx
            ; _call:
            ; call <_ret
        );

        let bytes = assembler.finalize()?;

        Ok(bytes)
    }
}