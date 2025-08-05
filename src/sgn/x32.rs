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

use crate::{obfuscation::x32::X32CodeAssembler, sgn::encoder::{SgnDecoderStub, ShikataGaNaiError}, x64_arch::registers::{get_save_random_general_purpose_register, RCX_FULL}};

impl SgnDecoderStub for X32CodeAssembler {
    fn get_sgn_decoder_stub(
        &self,
        seed: u8,
        payload_size: usize,
    ) -> Result<Vec<u8>, ShikataGaNaiError> {
        let mut assembler = VecAssembler::<X86Relocation>::new(0);
        let indexer_register = get_save_random_general_purpose_register(&[RCX_FULL]);
        let seed_register =
            get_save_random_general_purpose_register(&[RCX_FULL, indexer_register.clone()]);
        let indexer_register_id = indexer_register.quad as u8;
        let seed_register_id = seed_register.low as u8;

        dynasm!(assembler
            ; .arch x86
            ; call >_getip
            ; _getip:
            ; pop Rd(indexer_register_id)
            ; mov ecx, payload_size as i32
            ; mov Rb(seed_register_id), seed as i8
            ; _decode:
            ; xor BYTE [Rd(indexer_register_id) + ecx + 19], Rb(seed_register_id)
            ; add Rb(seed_register_id), BYTE [Rd(indexer_register_id) + ecx + 19]
            ; loop <_decode
            ; _data:
        );

        let bytes = assembler.finalize()?;

        Ok(bytes)
    }
}