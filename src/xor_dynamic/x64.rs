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

use crate::{
    obfuscation::x64::X64CodeAssembler,
    x64_arch::registers::{get_save_random_general_purpose_register, RBP_FULL, RCX_FULL, RSP_FULL},
    xor_dynamic::encoder::{XorDynamicEncoderError, XorDynamicStub},
};
use dynasmrt::{
    dynasm,
    x64::{X64Relocation},
    DynasmApi, DynasmError, DynasmLabelApi, VecAssembler,
};
use rand::Rng;

impl<RngType: Rng> XorDynamicStub for X64CodeAssembler<RngType> {
    fn get_decoder_stub(&mut self) -> Result<Vec<u8>, XorDynamicEncoderError> {
        let mut assembler = VecAssembler::<X64Relocation>::new(0);
        let link_register =
            get_save_random_general_purpose_register(&[RBP_FULL, RSP_FULL], &mut self.rng);
        let link_register_id = link_register.quad as u8;

        let jmp_register = get_save_random_general_purpose_register(
            &[RBP_FULL, RSP_FULL, link_register.clone()],
            &mut self.rng,
        );
        let jmp_register_id = jmp_register.quad as u8;

        let payload_indexer_register = get_save_random_general_purpose_register(
            &[
                RCX_FULL,
                RBP_FULL,
                RSP_FULL,
                link_register.clone(),
                jmp_register.clone(),
            ],
            &mut self.rng,
        );
        let payload_indexer_register_id = payload_indexer_register.quad as u8;

        let key_indexer_register = get_save_random_general_purpose_register(
            &[
                RCX_FULL,
                RBP_FULL,
                RSP_FULL,
                link_register.clone(),
                jmp_register.clone(),
                payload_indexer_register.clone(),
            ],
            &mut self.rng,
        );
        let key_indexer_register_id = key_indexer_register.quad as u8;

        dynasm!(assembler
            ; .arch x64
            ; jmp BYTE >call_label
            ; ret_label:
            ; pop Rq(link_register_id)
            ; push Rq(link_register_id)
            ; pop Rq(payload_indexer_register_id)
            ; mov al, 'A' as i8
            ; cld
            ; lp1:
            ; scasb
            ; jne BYTE <lp1
            ; push Rq(payload_indexer_register_id)
            ; pop Rq(jmp_register_id)
            ; lp2:
            ; push Rq(link_register_id)
            ; pop Rq(key_indexer_register_id)
            ; lp3:
            ; mov al, BYTE [Rq(key_indexer_register_id)]
            ; xor BYTE [Rq(payload_indexer_register_id)], al
            ; inc Rq(payload_indexer_register_id)
            ; inc Rq(key_indexer_register_id)
            ; cmp WORD [Rq(payload_indexer_register_id)], 0x4242
            ; je BYTE >jmp_label
            ; cmp BYTE [Rq(key_indexer_register_id)], 'A' as i8
            ; jne BYTE <lp3
            ; jmp BYTE <lp2
            ; jmp_label:
            ; jmp Rq(jmp_register_id)
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
