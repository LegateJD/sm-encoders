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

use std::collections::HashSet;

use dynasmrt::{dynasm, x86::X86Relocation, DynasmApi, DynasmLabelApi, VecAssembler};
use rand::Rng;

use crate::{obfuscation::x32::X32CodeAssembler, xor_dynamic::encoder::{XorDynamicEncoderError, XorDynamicStub, XorDecoderStub}};
use crate::x64_arch::registers::{get_save_random_general_purpose_register, RBP_FULL, RCX_FULL, RSP_FULL};

impl<RngType: Rng> XorDynamicStub for X32CodeAssembler<RngType> {
    fn get_xor_dynamic_decoder_stub(&mut self, _badchars: &HashSet<u8>) -> Result<XorDecoderStub, XorDynamicEncoderError> {
        let mut assembler = VecAssembler::<X86Relocation>::new(0);
        let link_register = get_save_random_general_purpose_register(&[RBP_FULL, RSP_FULL], &mut self.rng);
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
            ; .arch x86
            ; jmp >call_label
            ; ret_label:
            ; pop Rd(link_register_id)
            ; push Rd(link_register_id)
            ; pop Rd(payload_indexer_register_id)
            ; mov al, 'A' as i8
            ; cld
            ; lp1:
            ; scasb
            ; jne <lp1
            ; push Rd(payload_indexer_register_id)
            ; pop Rd(jmp_register_id)
            ; lp2:
            ; push Rd(link_register_id)
            ; pop Rd(key_indexer_register_id)
            ; lp3:
            ; mov al, BYTE [Rd(key_indexer_register_id)]
            ; xor BYTE [Rd(payload_indexer_register_id)], al
            ; inc Rd(payload_indexer_register_id)
            ; inc Rd(key_indexer_register_id)
            ; cmp WORD [Rd(payload_indexer_register_id)], 0x4242
            ; je >jmp_label
            ; cmp BYTE [Rd(key_indexer_register_id)], 'A' as i8
            ; jne <lp3
            ; jmp <lp2
            ; jmp_label:
            ; jmp Rd(jmp_register_id)
            ; call_label:
            ; call <ret_label
        );

        let bytes = assembler.finalize()?;

        Ok(XorDecoderStub {
            stub: bytes,
            key_terminator_stub: vec![0x41],
            payload_terminator_stub: vec![0x42, 0x42],
        })
    }
}