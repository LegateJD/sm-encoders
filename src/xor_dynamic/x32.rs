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