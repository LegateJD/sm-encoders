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

use iced_x86::code_asm::{
    ah, bh, bl, bpl, ch, cl, dh, dil, dl, r10b, r11b, r12b, r13b, r14b, r15b, r8b, r9b, rdx, sil, spl, AsmRegister64, AsmRegister8
};

#[derive(Debug, Clone, Copy)]
pub struct AsmRegister {
    pub full: AsmRegisterFull,
    pub extended: AsmRegisterExtended,
    pub high: AsmRegisterHigh,
    pub low: AsmRegisterLow,
}

impl PartialEq for AsmRegister {
    fn eq(&self, other: &Self) -> bool {
        self.full == other.full || self.extended == other.extended || self.high == other.high || self.low == other.low
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

impl From<AsmRegisterLow> for AsmRegister8 {
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
pub struct AsmRegisterExtended {
    register: Register,
}

impl AsmRegisterExtended {
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

impl fmt::Display for AsmRegisterExtended {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.register)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AsmRegisterFull {
    register: Register,
}

impl AsmRegisterFull {
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
            _ => unreachable!()
        }
    }
}

impl fmt::Display for AsmRegisterFull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.register)
    }
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

pub const AL: AsmRegisterLow = AsmRegisterLow::new(Register::AL);
pub const CL: AsmRegisterLow = AsmRegisterLow::new(Register::CL);
pub const DL: AsmRegisterLow = AsmRegisterLow::new(Register::DL);
pub const BL: AsmRegisterLow = AsmRegisterLow::new(Register::BL);
pub const AH: AsmRegisterLow = AsmRegisterLow::new(Register::AH);
pub const CH: AsmRegisterLow = AsmRegisterLow::new(Register::CH);
pub const DH: AsmRegisterLow = AsmRegisterLow::new(Register::DH);
pub const BH: AsmRegisterLow = AsmRegisterLow::new(Register::BH);
pub const SPL: AsmRegisterLow = AsmRegisterLow::new(Register::SPL);
pub const BPL: AsmRegisterLow = AsmRegisterLow::new(Register::BPL);
pub const SIL: AsmRegisterLow = AsmRegisterLow::new(Register::SIL);
pub const DIL: AsmRegisterLow = AsmRegisterLow::new(Register::DIL);
pub const R8B: AsmRegisterLow = AsmRegisterLow::new(Register::R8B);
pub const R9B: AsmRegisterLow = AsmRegisterLow::new(Register::R9B);
pub const R10B: AsmRegisterLow = AsmRegisterLow::new(Register::R10B);
pub const R11B: AsmRegisterLow = AsmRegisterLow::new(Register::R11B);
pub const R12B: AsmRegisterLow = AsmRegisterLow::new(Register::R12B);
pub const R13B: AsmRegisterLow = AsmRegisterLow::new(Register::R13B);
pub const R14B: AsmRegisterLow = AsmRegisterLow::new(Register::R14B);
pub const R15B: AsmRegisterLow = AsmRegisterLow::new(Register::R15B);

pub const AX: AsmRegisterHigh = AsmRegisterHigh::new(Register::AX);
pub const CX: AsmRegisterHigh = AsmRegisterHigh::new(Register::CX);
pub const DX: AsmRegisterHigh = AsmRegisterHigh::new(Register::DX);
pub const BX: AsmRegisterHigh = AsmRegisterHigh::new(Register::BX);
pub const SP: AsmRegisterHigh = AsmRegisterHigh::new(Register::SP);
pub const BP: AsmRegisterHigh = AsmRegisterHigh::new(Register::BP);
pub const SI: AsmRegisterHigh = AsmRegisterHigh::new(Register::SI);
pub const DI: AsmRegisterHigh = AsmRegisterHigh::new(Register::DI);
pub const R8W: AsmRegisterHigh = AsmRegisterHigh::new(Register::R8W);
pub const R9W: AsmRegisterHigh = AsmRegisterHigh::new(Register::R9W);
pub const R10W: AsmRegisterHigh = AsmRegisterHigh::new(Register::R10W);
pub const R11W: AsmRegisterHigh = AsmRegisterHigh::new(Register::R11W);
pub const R12W: AsmRegisterHigh = AsmRegisterHigh::new(Register::R12W);
pub const R13W: AsmRegisterHigh = AsmRegisterHigh::new(Register::R13W);
pub const R14W: AsmRegisterHigh = AsmRegisterHigh::new(Register::R14W);
pub const R15W: AsmRegisterHigh = AsmRegisterHigh::new(Register::R15W);

pub const EAX: AsmRegisterExtended = AsmRegisterExtended::new(Register::EAX);
pub const ECX: AsmRegisterExtended = AsmRegisterExtended::new(Register::ECX);
pub const EDX: AsmRegisterExtended = AsmRegisterExtended::new(Register::EDX);
pub const EBX: AsmRegisterExtended = AsmRegisterExtended::new(Register::EBX);
pub const ESP: AsmRegisterExtended = AsmRegisterExtended::new(Register::ESP);
pub const EBP: AsmRegisterExtended = AsmRegisterExtended::new(Register::EBP);
pub const ESI: AsmRegisterExtended = AsmRegisterExtended::new(Register::ESI);
pub const EDI: AsmRegisterExtended = AsmRegisterExtended::new(Register::EDI);
pub const R8D: AsmRegisterExtended = AsmRegisterExtended::new(Register::R8D);
pub const R9D: AsmRegisterExtended = AsmRegisterExtended::new(Register::R9D);
pub const R10D: AsmRegisterExtended = AsmRegisterExtended::new(Register::R10D);
pub const R11D: AsmRegisterExtended = AsmRegisterExtended::new(Register::R11D);
pub const R12D: AsmRegisterExtended = AsmRegisterExtended::new(Register::R12D);
pub const R13D: AsmRegisterExtended = AsmRegisterExtended::new(Register::R13D);
pub const R14D: AsmRegisterExtended = AsmRegisterExtended::new(Register::R14D);
pub const R15D: AsmRegisterExtended = AsmRegisterExtended::new(Register::R15D);

pub const RAX: AsmRegisterFull = AsmRegisterFull::new(Register::RAX);
pub const RCX: AsmRegisterFull = AsmRegisterFull::new(Register::RCX);
pub const RDX: AsmRegisterFull = AsmRegisterFull::new(Register::RDX);
pub const RBX: AsmRegisterFull = AsmRegisterFull::new(Register::RBX);
pub const RSP: AsmRegisterFull = AsmRegisterFull::new(Register::RSP);
pub const RBP: AsmRegisterFull = AsmRegisterFull::new(Register::RBP);
pub const RSI: AsmRegisterFull = AsmRegisterFull::new(Register::RSI);
pub const RDI: AsmRegisterFull = AsmRegisterFull::new(Register::RDI);
pub const R8: AsmRegisterFull = AsmRegisterFull::new(Register::R8);
pub const R9: AsmRegisterFull = AsmRegisterFull::new(Register::R9);
pub const R10: AsmRegisterFull = AsmRegisterFull::new(Register::R10);
pub const R11: AsmRegisterFull = AsmRegisterFull::new(Register::R11);
pub const R12: AsmRegisterFull = AsmRegisterFull::new(Register::R12);
pub const R13: AsmRegisterFull = AsmRegisterFull::new(Register::R13);
pub const R14: AsmRegisterFull = AsmRegisterFull::new(Register::R14);
pub const R15: AsmRegisterFull = AsmRegisterFull::new(Register::R15);

pub const RAX_Full: AsmRegister = AsmRegister {
        full: RAX,
        extended: EAX,
        high: AX,
        low: AL,
    }; 

    pub const RCX_Full: AsmRegister = AsmRegister {
        full: RCX,
        extended: ECX,
        high: CX,
        low: CL,
    }; 

pub const GENERAL_PURPOSE_REGISTERS_64_BIT: &[AsmRegister] = &[
    RAX_Full,
    AsmRegister {
        full: RBX,
        extended: EBX,
        high: BX,
        low: BL,
    },
    AsmRegister {
        full: RCX,
        extended: ECX,
        high: CX,
        low: CL,
    },
    AsmRegister {
        full: RDX,
        extended: EDX,
        high: DX,
        low: DL,
    },
    AsmRegister {
        full: RSI,
        extended: ESI,
        high: SI,
        low: SIL,
    },
    AsmRegister {
        full: RDI,
        extended: EDI,
        high: DI,
        low: DIL,
    },
    AsmRegister {
        full: R8,
        extended: R8D,
        high: R8W,
        low: R8B,
    },
    AsmRegister {
        full: R9,
        extended: R9D,
        high: R9W,
        low: R9B,
    },
    AsmRegister {
        full: R10,
        extended: R10D,
        high: R10W,
        low: R10B,
    },
    AsmRegister {
        full: R11,
        extended: R11D,
        high: R11W,
        low: R11B,
    },
    AsmRegister {
        full: R12,
        extended: R12D,
        high: R12W,
        low: R12B,
    },
    AsmRegister {
        full: R13,
        extended: R13D,
        high: R13W,
        low: R13B,
    },
    AsmRegister {
        full: R14,
        extended: R14D,
        high: R14W,
        low: R14B,
    },
    AsmRegister {
        full: R15,
        extended: R15D,
        high: R15W,
        low: R15B,
    },
];