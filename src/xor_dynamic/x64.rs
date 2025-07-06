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

use crate::{obfuscation::x64::X64CodeAssembler, xor_dynamic::encoder::{XorDynamicEncoderError, XorDynamicStub}};
use dynasmrt::{dynasm, x64::X64Relocation, DynasmApi, DynasmError, DynasmLabelApi, VecAssembler};

impl XorDynamicStub for X64CodeAssembler {
    fn get_decoder_stub(&self) -> Result<Vec<u8>, XorDynamicEncoderError> {
        let mut assembler = VecAssembler::<X64Relocation>::new(0);
        dynasm!(assembler
            ; .arch x64
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

impl From<DynasmError> for XorDynamicEncoderError {
    fn from(value: DynasmError) -> Self {
        XorDynamicEncoderError::AssemblerError
    }
}
