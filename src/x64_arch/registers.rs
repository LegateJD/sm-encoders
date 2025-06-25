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

use dynasmrt::{dynasm, x64::X64Relocation, x86::Rd, DynasmApi, DynasmLabelApi, VecAssembler};
use iced_x86::code_asm::{
    ah, bh, bl, bpl, ch, cl, dh, dil, dl, r10b, r11b, r12b, r13b, r14b, r15b, r8b, r9b, rdx, sil,
    spl, AsmRegister64, AsmRegister8,
};
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

#[derive(Debug, Clone, Copy)]
pub struct AsmRegisterLow {
    register: Register,
}

impl AsmRegisterLow {
    #[must_use]
    #[inline]
    pub(crate) const fn new(register: Register) -> Self {
        Self { register }
    }

    #[must_use]
    #[inline]
    pub(crate) fn register(&self) -> Register {
        self.register
    }
}

impl From<AsmRegisterLow> for Rb {
    fn from(value: AsmRegisterLow) -> Self {
        match value.register {
            Register::AL => ah,
            Register::CL => cl,
            Register::DL => dl,
            Register::BL => bl,
            Register::AH => ah,
            Register::CH => ch,
            Register::DH => dh,
            Register::BH => bh,
            Register::SPL => spl,
            Register::BPL => bpl,
            Register::SIL => sil,
            Register::DIL => dil,
            Register::R8B => r8b,
            Register::R9B => r9b,
            Register::R10B => r10b,
            Register::R11B => r11b,
            Register::R12B => r12b,
            Register::R13B => r13b,
            Register::R14B => r14b,
            Register::R15B => r15b,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for AsmRegisterLow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.register)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AsmRegisterHigh {
    register: Register,
}

impl AsmRegisterHigh {
    #[must_use]
    #[inline]
    pub(crate) const fn new(register: Register) -> Self {
        Self { register }
    }

    #[must_use]
    #[inline]
    pub(crate) fn register(&self) -> Register {
        self.register
    }
}

impl fmt::Display for AsmRegisterHigh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.register)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AsmRegisterDouble {
    register: Register,
}

impl AsmRegisterDouble {
    #[must_use]
    #[inline]
    pub(crate) const fn new(register: Register) -> Self {
        Self { register }
    }

    #[must_use]
    #[inline]
    pub(crate) fn register(&self) -> Register {
        self.register
    }
}

impl fmt::Display for AsmRegisterDouble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.register)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AsmRegisterQuad {
    register: Register,
}

impl AsmRegisterQuad {
    #[must_use]
    #[inline]
    pub(crate) const fn new(register: Register) -> Self {
        Self { register }
    }

    #[must_use]
    #[inline]
    pub(crate) fn register(&self) -> Register {
        self.register
    }
}

impl From<AsmRegisterLow> for AsmRegister64 {
    fn from(value: AsmRegisterLow) -> Self {
        match value.register {
            Register::RAX => rax,
            Register::RCX => rcx,
            Register::RDX => rdx,
            Register::RBX => todo!(),
            Register::RSP => todo!(),
            Register::RBP => todo!(),
            Register::RSI => todo!(),
            Register::RDI => todo!(),
            Register::R8 => todo!(),
            Register::R9 => todo!(),
            Register::R10 => todo!(),
            Register::R11 => todo!(),
            Register::R12 => todo!(),
            Register::R13 => todo!(),
            Register::R14 => todo!(),
            Register::R15 => todo!(),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for AsmRegisterQuad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.register)
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Register {
    None = 0,
    AL = 1,
    CL = 2,
    DL = 3,
    BL = 4,
    AH = 5,
    CH = 6,
    DH = 7,
    BH = 8,
    SPL = 9,
    BPL = 10,
    SIL = 11,
    DIL = 12,
    R8B = 13,
    R9B = 14,
    R10B = 15,
    R11B = 16,
    R12B = 17,
    R13B = 18,
    R14B = 19,
    R15B = 20,
    AX = 21,
    CX = 22,
    DX = 23,
    BX = 24,
    SP = 25,
    BP = 26,
    SI = 27,
    DI = 28,
    R8W = 29,
    R9W = 30,
    R10W = 31,
    R11W = 32,
    R12W = 33,
    R13W = 34,
    R14W = 35,
    R15W = 36,
    EAX = 37,
    ECX = 38,
    EDX = 39,
    EBX = 40,
    ESP = 41,
    EBP = 42,
    ESI = 43,
    EDI = 44,
    R8D = 45,
    R9D = 46,
    R10D = 47,
    R11D = 48,
    R12D = 49,
    R13D = 50,
    R14D = 51,
    R15D = 52,
    RAX = 53,
    RCX = 54,
    RDX = 55,
    RBX = 56,
    RSP = 57,
    RBP = 58,
    RSI = 59,
    RDI = 60,
    R8 = 61,
    R9 = 62,
    R10 = 63,
    R11 = 64,
    R12 = 65,
    R13 = 66,
    R14 = 67,
    R15 = 68,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register::None => write!(f, "None"),
            Register::AL => write!(f, "AL"),
            Register::CL => write!(f, "CL"),
            Register::DL => write!(f, "DL"),
            Register::BL => write!(f, "BL"),
            Register::AH => write!(f, "AH"),
            Register::CH => write!(f, "CH"),
            Register::DH => write!(f, "DH"),
            Register::BH => write!(f, "BH"),
            Register::SPL => write!(f, "SPL"),
            Register::BPL => write!(f, "BPL"),
            Register::SIL => write!(f, "SIL"),
            Register::DIL => write!(f, "DIL"),
            Register::R8B => write!(f, "R8B"),
            Register::R9B => write!(f, "R9B"),
            Register::R10B => write!(f, "R10B"),
            Register::R11B => write!(f, "R11B"),
            Register::R12B => write!(f, "R12B"),
            Register::R13B => write!(f, "R13B"),
            Register::R14B => write!(f, "R14B"),
            Register::R15B => write!(f, "R15B"),
            Register::AX => write!(f, "AX"),
            Register::CX => write!(f, "CX"),
            Register::DX => write!(f, "DX"),
            Register::BX => write!(f, "BX"),
            Register::SP => write!(f, "SP"),
            Register::BP => write!(f, "BP"),
            Register::SI => write!(f, "SI"),
            Register::DI => write!(f, "DI"),
            Register::R8W => write!(f, "R8W"),
            Register::R9W => write!(f, "R9W"),
            Register::R10W => write!(f, "R10W"),
            Register::R11W => write!(f, "R11W"),
            Register::R12W => write!(f, "R12W"),
            Register::R13W => write!(f, "R13W"),
            Register::R14W => write!(f, "R14W"),
            Register::R15W => write!(f, "R15W"),
            Register::EAX => write!(f, "EAX"),
            Register::ECX => write!(f, "ECX"),
            Register::EDX => write!(f, "EDX"),
            Register::EBX => write!(f, "EBX"),
            Register::ESP => write!(f, "ESP"),
            Register::EBP => write!(f, "EBP"),
            Register::ESI => write!(f, "ESI"),
            Register::EDI => write!(f, "EDI"),
            Register::R8D => write!(f, "R8D"),
            Register::R9D => write!(f, "R9D"),
            Register::R10D => write!(f, "R10D"),
            Register::R11D => write!(f, "R11D"),
            Register::R12D => write!(f, "R12D"),
            Register::R13D => write!(f, "R13D"),
            Register::R14D => write!(f, "R14D"),
            Register::R15D => write!(f, "R15D"),
            Register::RAX => write!(f, "RAX"),
            Register::RCX => write!(f, "RCX"),
            Register::RDX => write!(f, "RDX"),
            Register::RBX => write!(f, "RBX"),
            Register::RSP => write!(f, "RSP"),
            Register::RBP => write!(f, "RBP"),
            Register::RSI => write!(f, "RSI"),
            Register::RDI => write!(f, "RDI"),
            Register::R8 => write!(f, "R8"),
            Register::R9 => write!(f, "R9"),
            Register::R10 => write!(f, "R10"),
            Register::R11 => write!(f, "R11"),
            Register::R12 => write!(f, "R12"),
            Register::R13 => write!(f, "R13"),
            Register::R14 => write!(f, "R14"),
            Register::R15 => write!(f, "R15"),
        }
    }
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
    low: LowRegister::SIL, // Note: For SI, DI, BP, SP, the 8-bit registers are SIL, DIL, BPL, SPL in x86-64
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
