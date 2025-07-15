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

use dynasmrt::{aarch64::Aarch64Relocation, dynasm, x64::X64Relocation, x86::X86Relocation, DynasmApi, DynasmError, DynasmLabelApi, VecAssembler};

use crate::{obfuscation::{aarch64::AArch64CodeAssembler, x32::X32CodeAssembler}, sgn::encoder::{SgnDecoderStub, ShikataGaNaiError}};
use crate::arm64::registers::{get_random_general_purpose_register, get_safe_random_general_purpose_register};

impl SgnDecoderStub for AArch64CodeAssembler {
    fn get_sgn_decoder_stub(
        &self,
        seed: u8,
        payload_size: usize,
    ) -> Result<Vec<u8>, ShikataGaNaiError> {
        let mut assembler = VecAssembler::<Aarch64Relocation>::new(0);
        let indexer_register = get_random_general_purpose_register();
        let seed_register =
            get_safe_random_general_purpose_register(&[indexer_register.clone()]);
        let payload_register =
            get_safe_random_general_purpose_register(&[indexer_register.clone(), seed_register.clone()]);
        let xor_result_register =
            get_safe_random_general_purpose_register(&[indexer_register.clone(), seed_register.clone(), payload_register.clone()]);
        let add_result_register =
            get_safe_random_general_purpose_register(&[indexer_register.clone(), seed_register.clone(), payload_register.clone(), xor_result_register.clone()]);
        let indexer_register_id = indexer_register.x as u32;
        let seed_register_id = seed_register.x as u32;
        let payload_siez_register_id = payload_register.x as u32;
        let xor_result_register_register_id = xor_result_register.x as u32;
        let add_result_register_register_id = add_result_register.x as u32;

        dynasm!(assembler
            ; .arch aarch64
            ; mov W(seed_register_id), seed as u32
            ; mov X(payload_siez_register_id), payload_size as u64
            ; adr X(indexer_register_id), >_data_sub1
            ; eor W(xor_result_register_register_id), W(xor_result_register_register_id), W(seed_register_id)
            ; strb W(xor_result_register_register_id), [X(indexer_register_id), X(payload_siez_register_id)]
            ; ldrb W(add_result_register_register_id), [X(indexer_register_id), X(payload_siez_register_id)]
            ; add W(seed_register_id), W(seed_register_id), W(add_result_register_register_id)
            ; subs X(payload_siez_register_id), XSP(payload_siez_register_id), 1
            ; b.ne >_decode
            ; _decode:
            ; ldrb W(xor_result_register_register_id), [X(indexer_register_id), X(payload_siez_register_id)]
            ; _data_sub1:
        );

        /*dynasm!(assembler
            ; .arch aarch64
            ; mov w3, seed as u32
            ; mov x2, payload_size as u64
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
        );*/

        let bytes = assembler.finalize()?;

        Ok(bytes)
    }
}