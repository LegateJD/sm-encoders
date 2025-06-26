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

use dynasmrt::{dynasm, x64::X64Relocation, x86::Rd, DynasmApi, DynasmLabelApi, VecAssembler};
use rand::seq::IndexedRandom;

#[derive(Debug, Clone, Copy)]
pub struct AsmRegister {
    pub quad: QuadRegister,
    pub double: DoubleRegister,
    pub word: WordRegister,
    pub low: LowRegister,
}

impl PartialEq for AsmRegister {
    fn eq(&self, other: &Self) -> bool {
        self.quad == other.quad
            || self.double == other.double
            || self.word == other.word
            || self.low == other.low
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LowRegister {
    AL = 0,
    CL = 1,
    DL = 2,
    BL = 3,
    SPL = 4,
    BPL = 5,
    SIL = 6,
    DIL = 7,
    R8B = 8,
    R9B = 9,
    R10B = 10,
    R11B = 11,
    R12B = 12,
    R13B = 13,
    R14B = 14,
    R15B = 15,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HighRegister {
    AH = 4,
    CH = 5,
    DH = 6,
    BH = 7,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WordRegister {
    AX = 0,
    CX = 1,
    DX = 2,
    BX = 3,
    SP = 4,
    BP = 5,
    SI = 6,
    DI = 7,
    R8W = 8,
    R9W = 9,
    R10W = 10,
    R11W = 11,
    R12W = 12,
    R13W = 13,
    R14W = 14,
    R15W = 15,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DoubleRegister {
    EAX = 0,
    ECX = 1,
    EDX = 2,
    EBX = 3,
    ESP = 4,
    EBP = 5,
    ESI = 6,
    EDI = 7,
    R8D = 8,
    R9D = 9,
    R10D = 10,
    R11D = 11,
    R12D = 12,
    R13D = 13,
    R14D = 14,
    R15D = 15,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum QuadRegister {
    RAX = 0,
    RCX = 1,
    RDX = 2,
    RBX = 3,
    RSP = 4,
    RBP = 5,
    RSI = 6,
    RDI = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

pub const RAX_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::RAX,
    double: DoubleRegister::EAX,
    word: WordRegister::AX,
    low: LowRegister::AL,
};

pub const RBX_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::RBX,
    double: DoubleRegister::EBX,
    word: WordRegister::BX,
    low: LowRegister::BL,
};

pub const RCX_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::RCX,
    double: DoubleRegister::ECX,
    word: WordRegister::CX,
    low: LowRegister::CL,
};

pub const RDX_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::RDX,
    double: DoubleRegister::EDX,
    word: WordRegister::DX,
    low: LowRegister::DL,
};

pub const RSI_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::RSI,
    double: DoubleRegister::ESI,
    word: WordRegister::SI,
    low: LowRegister::SIL,
};

pub const RDI_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::RDI,
    double: DoubleRegister::EDI,
    word: WordRegister::DI,
    low: LowRegister::DIL,
};

pub const RBP_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::RBP,
    double: DoubleRegister::EBP,
    word: WordRegister::BP,
    low: LowRegister::BPL,
};

pub const RSP_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::RSP,
    double: DoubleRegister::ESP,
    word: WordRegister::SP,
    low: LowRegister::SPL,
};

pub const R8_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::R8,
    double: DoubleRegister::R8D,
    word: WordRegister::R8W,
    low: LowRegister::R8B,
};

pub const R9_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::R9,
    double: DoubleRegister::R9D,
    word: WordRegister::R9W,
    low: LowRegister::R9B,
};

pub const R10_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::R10,
    double: DoubleRegister::R10D,
    word: WordRegister::R10W,
    low: LowRegister::R10B,
};

pub const R11_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::R11,
    double: DoubleRegister::R11D,
    word: WordRegister::R11W,
    low: LowRegister::R11B,
};

pub const R12_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::R12,
    double: DoubleRegister::R12D,
    word: WordRegister::R12W,
    low: LowRegister::R12B,
};

pub const R13_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::R13,
    double: DoubleRegister::R13D,
    word: WordRegister::R13W,
    low: LowRegister::R13B,
};

pub const R14_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::R14,
    double: DoubleRegister::R14D,
    word: WordRegister::R14W,
    low: LowRegister::R14B,
};

pub const R15_FULL: AsmRegister = AsmRegister {
    quad: QuadRegister::R15,
    double: DoubleRegister::R15D,
    word: WordRegister::R15W,
    low: LowRegister::R15B,
};

pub const GENERAL_PURPOSE_REGISTERS_64_BIT: &[AsmRegister] = &[
    RAX_FULL, RBX_FULL, RCX_FULL, RDX_FULL, RSI_FULL, RDI_FULL, RBP_FULL, RSP_FULL, R8_FULL,
    R9_FULL, R10_FULL, R11_FULL, R12_FULL, R13_FULL, R14_FULL, R15_FULL,
];

pub fn get_save_random_general_purpose_register(excludes: &[AsmRegister]) -> &'static AsmRegister {
    let mut rng = rand::rng();
    let mut filtered = vec![];

    for reg in GENERAL_PURPOSE_REGISTERS_64_BIT.iter() {
        if !excludes.contains(&reg)
            && !excludes.contains(&reg)
            && !excludes.contains(&reg)
            && !excludes.contains(&reg)
        {
            filtered.push(reg);
        }
    }

    let register = filtered.choose(&mut rng).unwrap();

    *register
}

pub fn get_random_general_purpose_register() -> &'static AsmRegister {
    let mut rng = rand::rng();
    let register = GENERAL_PURPOSE_REGISTERS_64_BIT.choose(&mut rng).unwrap();

    register
}
