use crate::{obfuscation::x64::X64CodeAssembler, xor_dynamic::encoder::XorDynamicStub};
use dynasmrt::{dynasm, x64::X64Relocation, DynasmApi, DynasmLabelApi, VecAssembler};

impl XorDynamicStub for X64CodeAssembler {
    fn get_decoder_stub(&self, payload_size: usize) -> Result<Vec<u8>, anyhow::Error> {
        let mut assembler = VecAssembler::<X64Relocation>::new(0);
        dynasm!(assembler
            ; jmp BYTE >call_label
            ; ret_label:
            ; pop rbx
            ; push rbx
            ; pop rdi
            ; mov al, 'A' as i8
            ; cld
            ; lp1:
            ; scasb
            ; jne BYTE <lp1
            ; push rdi
            ; pop rcx
            ; lp2:
            ; push rbx
            ; pop rsi
            ; lp3:
            ; mov al, BYTE [rsi]
            ; xor BYTE [rdi], al
            ; inc rdi
            ; inc rsi
            ; cmp WORD [rdi], 0x4242
            ; je BYTE >jmp_label
            ; cmp BYTE [rsi], 'A' as i8
            ; jne BYTE <lp3
            ; jmp BYTE <lp2
            ; jmp_label:
            ; jmp rcx
            ; call_label:
            ; call <ret_label
        );

        let bytes = assembler.finalize()?;

        Ok(bytes)
    }
}
