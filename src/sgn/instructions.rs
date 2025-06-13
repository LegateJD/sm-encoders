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

use std::fmt;

use rand::{
    distr::{Distribution, StandardUniform},
    Rng,
};

#[derive(Debug, Clone, Copy)]
pub enum SchemaInstruction {
    XOR,
    SUB,
    ADD,
    ROL,
    ROR,
    NOT
}

impl fmt::Display for SchemaInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchemaInstruction::XOR => write!(f, "XOR"),
            SchemaInstruction::SUB => write!(f, "SUB"),
            SchemaInstruction::ADD => write!(f, "ADD"),
            SchemaInstruction::ROL => write!(f, "ROL"),
            SchemaInstruction::ROR => write!(f, "ROR"),
            SchemaInstruction::NOT => write!(f, "NOT"),
        }
    }
}

impl Distribution<SchemaInstruction> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SchemaInstruction {
        let index: u8 = rng.random_range(0..=5);
        match index {
            0 => SchemaInstruction::XOR,
            1 => SchemaInstruction::SUB,
            2 => SchemaInstruction::ADD,
            3 => SchemaInstruction::ROL,
            4 => SchemaInstruction::ROR,
            _ => SchemaInstruction::NOT,
        }
    }
}

pub const CONDITIONAL_JUMP_MNEMONICS: &[&'static str] = &[
    "JAE", "JA", "JBE", "JB", "JC", "JE", "JGE", "JG", "JLE", "JL", "JNAE", "JNA", "JNBE", "JNB",
    "JNC", "JNE", "JNGE", "JNG", "JNLE", "JNL", "JNO", "JNP", "JNS", "JNZ", "JO", "JPE", "JPO",
    "JP", "JS", "JZ",
];

pub const SAFE_GARBAGE_INSTRUCTIONS: &[&'static str] = &[
    ";", // no instruction (empty)
    "NOP",
    "CLD",
    "CLC",
    "CMC",
    "WAIT",
    "FNOP",
    "FXAM",
    "FTST",
    "JMP 2",
    "ROL {R},0",
    "ROR {R},0",
    "SHL {R},0",
    "SHR {R},0",
    "RCL {R},0",
    "RCR {R},0",
    "SAL {R},0",
    "SAR {R},0",
    "XOR {R},0",
    "SUB {R},0",
    "ADD {R},0",
    "AND {R},{R}",
    "OR {R},{R}",
    "BT {R},{R}",
    "CMP {R},{R}",
    "MOV {R},{R}",
    "XCHG {R},{R}",
    "TEST {R},{R}",
    "CMOVA {R},{R}",
    "CMOVB {R},{R}",
    "CMOVC {R},{R}",
    "CMOVE {R},{R}",
    "CMOVG {R},{R}",
    "CMOVL {R},{R}",
    "CMOVO {R},{R}",
    "CMOVP {R},{R}",
    "CMOVS {R},{R}",
    "CMOVZ {R},{R}",
    "CMOVAE {R},{R}",
    "CMOVGE {R},{R}",
    "CMOVLE {R},{R}",
    "CMOVNA {R},{R}",
    "CMOVNB {R},{R}",
    "CMOVNC {R},{R}",
    "CMOVNE {R},{R}",
    "CMOVNG {R},{R}",
    "CMOVNL {R},{R}",
    "CMOVNO {R},{R}",
    "CMOVNP {R},{R}",
    "CMOVNS {R},{R}",
    "CMOVNZ {R},{R}",
    "CMOVPE {R},{R}",
    "CMOVPO {R},{R}",
    "CMOVBE {R},{R}",
    "CMOVNAE {R},{R}",
    "CMOVNBE {R},{R}",
    "CMOVNLE {R},{R}",
    "CMOVNGE {R},{R}",
    // Recursion starts here...
    "JMP {L};{G};{L}:",
    "NOT {R};{G};NOT {R}",
    "NEG {R};{G};NEG {R}",
    "INC {R};{G};DEC {R}",
    "DEC {R};{G};INC {R}",
    // "PUSH {R};{G};POP {R}",
    // "BSWAP {R};{G};BSWAP {R}",
    "ADD {R},{K};{G};SUB {R},{K}",
    "SUB {R},{K};{G};ADD {R},{K}",
    "ROR {R},{K};{G};ROL {R},{K}",
    "ROL {R},{K};{G};ROR {R},{K}",
];
