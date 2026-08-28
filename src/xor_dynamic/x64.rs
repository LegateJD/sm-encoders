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

use crate::{
    obfuscation::x64::X64CodeAssembler, x64_arch::registers::{RAX_FULL, RBP_FULL, RCX_FULL, RDI_FULL, RSP_FULL, get_save_random_general_purpose_register}, xor_dynamic::encoder::{XorDecoderStub, XorDynamicEncoderError, XorDynamicStub},
};
use dynasmrt::{
    dynasm,
    x64::{X64Relocation},
    DynasmApi, DynasmError, DynasmLabelApi, VecAssembler,
};
use rand::Rng;

impl<RngType: Rng> XorDynamicStub for X64CodeAssembler<RngType> {
    fn get_xor_dynamic_decoder_stub(&mut self, _badchars: &HashSet<u8>) -> Result<XorDecoderStub, XorDynamicEncoderError> {
        let link_register =
            get_save_random_general_purpose_register(&[RBP_FULL, RSP_FULL, RDI_FULL, RAX_FULL], &mut self.rng);
        let link_register_id = link_register.quad as u8;

        let jmp_register = get_save_random_general_purpose_register(
            &[RBP_FULL, RSP_FULL, RDI_FULL, RAX_FULL, link_register.clone()],
            &mut self.rng,
        );
        let jmp_register_id = jmp_register.quad as u8;

        let payload_indexer_register = get_save_random_general_purpose_register(
            &[
                RCX_FULL,
                RBP_FULL,
                RSP_FULL,
                RDI_FULL,
                RAX_FULL,
                link_register.clone(),
                jmp_register.clone(),
            ],
            &mut self.rng,
        );

        let key_indexer_register = get_save_random_general_purpose_register(
            &[
                RCX_FULL,
                RBP_FULL,
                RSP_FULL,
                RDI_FULL,
                RAX_FULL,
                link_register.clone(),
                jmp_register.clone(),
                payload_indexer_register.clone(),
            ],
            &mut self.rng,
        );
        let key_indexer_register_id = key_indexer_register.quad as u8;

        let mut assembler = VecAssembler::<X64Relocation>::new(0);
        dynasm!(assembler
            ; .arch x64
            ; jmp BYTE >call_label
            ; ret_label:
            ; pop Rq(link_register_id)
            ; push Rq(link_register_id)
            ; pop rdi
            ; mov al, 'A' as i8
            ; cld
            ; lp1:
            ; scasb
            ; jne BYTE <lp1
            ; push rdi
            ; pop Rq(jmp_register_id)
            ; lp2:
            ; push Rq(link_register_id)
            ; pop Rq(key_indexer_register_id)
            ; lp3:
            ; .bytes emit_mov_al_mem(key_indexer_register_id)
            ; xor BYTE [rdi], al
            ; inc rdi
            ; inc Rq(key_indexer_register_id)
            ; cmp WORD [rdi], 0x4242
            ; je BYTE >jmp_label
            ; .bytes emit_cmp_byte_mem_imm8(key_indexer_register_id, b'A')
            ; jne BYTE <lp3
            ; jmp BYTE <lp2
            ; jmp_label:
            ; jmp Rq(jmp_register_id)
            ; call_label:
            ; call <ret_label
        );

        let bytes = assembler.finalize()?;
        let hashset = bytes.into_iter().collect::<HashSet<u8>>();
        //hashset.extend(badchars.iter().cloned());

        let first_char = (0..=u8::MAX)
            .find(|x| !hashset.contains(x))
            .ok_or(XorDynamicEncoderError::NoAvailableByteForStubTerminator)?;

        let sec_char = (0..=u8::MAX)
            .find(|x| !hashset.contains(x) && *x != first_char)
            .ok_or(XorDynamicEncoderError::NoAvailableByteForStubTerminator)?;

        let compare = (sec_char as u16) << 8 | (sec_char as u16);

        let mut assembler = VecAssembler::<X64Relocation>::new(0);
        dynasm!(assembler
            ; .arch x64
            ; jmp BYTE >call_label
            ; ret_label:
            ; pop Rq(link_register_id)
            ; push Rq(link_register_id)
            ; pop rdi
            ; mov al, first_char as i8
            ; cld
            ; lp1:
            ; scasb
            ; jne BYTE <lp1
            ; push rdi
            ; pop Rq(jmp_register_id)
            ; lp2:
            ; push Rq(link_register_id)
            ; pop Rq(key_indexer_register_id)
            ; lp3:
            ; .bytes emit_mov_al_mem(key_indexer_register_id)
            ; xor BYTE [rdi], al
            ; inc rdi
            ; inc Rq(key_indexer_register_id)
            ; cmp WORD [rdi], compare as i16
            ; je BYTE >jmp_label
            ; .bytes emit_cmp_byte_mem_imm8(key_indexer_register_id, first_char)
            ; jne BYTE <lp3
            ; jmp BYTE <lp2
            ; jmp_label:
            ; jmp Rq(jmp_register_id)
            ; call_label:
            ; call <ret_label
        );

        let bytes = assembler.finalize()?;
        let decoder_stub = XorDecoderStub {
            stub: bytes,
            key_terminator_stub: vec![first_char],
            payload_terminator_stub: vec![sec_char, sec_char],
        };

        Ok(decoder_stub)
    }
}

impl From<DynasmError> for XorDynamicEncoderError {
    fn from(_value: DynasmError) -> Self {
        XorDynamicEncoderError::AssemblerError
    }
}

/// ModR/M (+ optional SIB/disp) for a `[base]` memory operand with the given
/// ModR/M.reg field, using the shortest form legal for that base register.
///
/// rsp/r12 (rm=100) require a SIB byte; rbp/r13 (rm=101) require a disp8=0.
fn modrm_mem(reg_field: u8, base_id: u8) -> Vec<u8> {
    match base_id & 0x7 {
        0b100 => vec![(reg_field << 3) | 0b100, 0x24],
        0b101 => vec![0x40 | (reg_field << 3) | 0b101, 0x00],
        low => vec![(reg_field << 3) | low],
    }
}

/// `mov al, BYTE [base]`
fn emit_mov_al_mem(base_id: u8) -> Vec<u8> {
    let mut out = Vec::new();
    if base_id >= 8 {
        out.push(0x41); // REX.B
    }
    out.push(0x8a);
    out.extend(modrm_mem(0, base_id));
    out
}

/// `xor BYTE [base], al`
fn emit_xor_mem_al(base_id: u8) -> Vec<u8> {
    let mut out = Vec::new();
    if base_id >= 8 {
        out.push(0x41); // REX.B
    }
    out.push(0x30);
    out.extend(modrm_mem(0, base_id));
    out
}

/// `cmp WORD [base], imm16`
fn emit_cmp_word_mem_imm16(base_id: u8, imm: u16) -> Vec<u8> {
    let mut out = vec![0x66]; // operand-size prefix
    if base_id >= 8 {
        out.push(0x41); // REX.B
    }
    out.push(0x81);
    out.extend(modrm_mem(7, base_id));
    out.extend_from_slice(&imm.to_le_bytes());
    out
}

/// `cmp BYTE [base], imm8`
fn emit_cmp_byte_mem_imm8(base_id: u8, imm: u8) -> Vec<u8> {
    let mut out = Vec::new();
    if base_id >= 8 {
        out.push(0x41); // REX.B
    }
    out.push(0x80);
    out.extend(modrm_mem(7, base_id));
    out.push(imm);
    out
}
