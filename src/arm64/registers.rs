use rand::seq::IndexedRandom;

#[derive(Debug, Clone, Copy)]
pub struct Arm64Register {
    pub x: XRegister,    // 64-bit register
    pub w: WRegister,    // 32-bit register
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum XRegister {
    X0 = 0,   // First parameter, return value
    X1 = 1,   // Second parameter, return value for larger returns
    X2 = 2,   // Third parameter
    X3 = 3,   // Fourth parameter
    X4 = 4,   // Fifth parameter
    X5 = 5,   // Sixth parameter
    X6 = 6,   // Seventh parameter
    X7 = 7,   // Eighth parameter
    X8 = 8,   // Indirect result location register
    X9 = 9,   // Temporary register
    X10 = 10, // Temporary register
    X11 = 11, // Temporary register
    X12 = 12, // Temporary register
    X13 = 13, // Temporary register
    X14 = 14, // Temporary register
    X15 = 15, // Temporary register
    X16 = 16, // IP0: Intra-procedure-call temporary register
    X17 = 17, // IP1: Intra-procedure-call temporary register
    X18 = 18, // Platform register (reserved)
    X19 = 19, // Callee-saved register
    X20 = 20, // Callee-saved register
    X21 = 21, // Callee-saved register
    X22 = 22, // Callee-saved register
    X23 = 23, // Callee-saved register
    X24 = 24, // Callee-saved register
    X25 = 25, // Callee-saved register
    X26 = 26, // Callee-saved register
    X27 = 27, // Callee-saved register
    X28 = 28, // Callee-saved register
    X29 = 29, // Frame pointer
    X30 = 30, // Link register
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum WRegister {
    W0 = 0,
    W1 = 1,
    W2 = 2,
    W3 = 3,
    W4 = 4,
    W5 = 5,
    W6 = 6,
    W7 = 7,
    W8 = 8,
    W9 = 9,
    W10 = 10,
    W11 = 11,
    W12 = 12,
    W13 = 13,
    W14 = 14,
    W15 = 15,
    W16 = 16,
    W17 = 17,
    W18 = 18,
    W19 = 19,
    W20 = 20,
    W21 = 21,
    W22 = 22,
    W23 = 23,
    W24 = 24,
    W25 = 25,
    W26 = 26,
    W27 = 27,
    W28 = 28,
    W29 = 29,
    W30 = 30,
}

impl PartialEq for Arm64Register {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

pub const X0_FULL: Arm64Register = Arm64Register { x: XRegister::X0, w: WRegister::W0 };
pub const X1_FULL: Arm64Register = Arm64Register { x: XRegister::X1, w: WRegister::W1 };
pub const X2_FULL: Arm64Register = Arm64Register { x: XRegister::X2, w: WRegister::W2 };
pub const X3_FULL: Arm64Register = Arm64Register { x: XRegister::X3, w: WRegister::W3 };
pub const X4_FULL: Arm64Register = Arm64Register { x: XRegister::X4, w: WRegister::W4 };
pub const X5_FULL: Arm64Register = Arm64Register { x: XRegister::X5, w: WRegister::W5 };
pub const X6_FULL: Arm64Register = Arm64Register { x: XRegister::X6, w: WRegister::W6 };
pub const X7_FULL: Arm64Register = Arm64Register { x: XRegister::X7, w: WRegister::W7 };
pub const X8_FULL: Arm64Register = Arm64Register { x: XRegister::X8, w: WRegister::W8 };
pub const X9_FULL: Arm64Register = Arm64Register { x: XRegister::X9, w: WRegister::W9 };
pub const X10_FULL: Arm64Register = Arm64Register { x: XRegister::X10, w: WRegister::W10 };
pub const X11_FULL: Arm64Register = Arm64Register { x: XRegister::X11, w: WRegister::W11 };
pub const X12_FULL: Arm64Register = Arm64Register { x: XRegister::X12, w: WRegister::W12 };
pub const X13_FULL: Arm64Register = Arm64Register { x: XRegister::X13, w: WRegister::W13 };
pub const X14_FULL: Arm64Register = Arm64Register { x: XRegister::X14, w: WRegister::W14 };
pub const X15_FULL: Arm64Register = Arm64Register { x: XRegister::X15, w: WRegister::W15 };
pub const X16_FULL: Arm64Register = Arm64Register { x: XRegister::X16, w: WRegister::W16 };
pub const X17_FULL: Arm64Register = Arm64Register { x: XRegister::X17, w: WRegister::W17 };
pub const X18_FULL: Arm64Register = Arm64Register { x: XRegister::X18, w: WRegister::W18 };
pub const X19_FULL: Arm64Register = Arm64Register { x: XRegister::X19, w: WRegister::W19 };
pub const X20_FULL: Arm64Register = Arm64Register { x: XRegister::X20, w: WRegister::W20 };
pub const X21_FULL: Arm64Register = Arm64Register { x: XRegister::X21, w: WRegister::W21 };
pub const X22_FULL: Arm64Register = Arm64Register { x: XRegister::X22, w: WRegister::W22 };
pub const X23_FULL: Arm64Register = Arm64Register { x: XRegister::X23, w: WRegister::W23 };
pub const X24_FULL: Arm64Register = Arm64Register { x: XRegister::X24, w: WRegister::W24 };
pub const X25_FULL: Arm64Register = Arm64Register { x: XRegister::X25, w: WRegister::W25 };
pub const X26_FULL: Arm64Register = Arm64Register { x: XRegister::X26, w: WRegister::W26 };
pub const X27_FULL: Arm64Register = Arm64Register { x: XRegister::X27, w: WRegister::W27 };
pub const X28_FULL: Arm64Register = Arm64Register { x: XRegister::X28, w: WRegister::W28 };
pub const X29_FULL: Arm64Register = Arm64Register { x: XRegister::X29, w: WRegister::W29 }; // Frame pointer
pub const X30_FULL: Arm64Register = Arm64Register { x: XRegister::X30, w: WRegister::W30 }; // Link register

pub const GENERAL_PURPOSE_REGISTERS: &[Arm64Register] = &[
    X0_FULL, X1_FULL, X2_FULL, X3_FULL, X4_FULL, X5_FULL, X6_FULL, X7_FULL,
    X8_FULL, X9_FULL, X10_FULL, X11_FULL, X12_FULL, X13_FULL, X14_FULL, X15_FULL,
    X16_FULL, X17_FULL, X18_FULL, X19_FULL, X20_FULL, X21_FULL, X22_FULL, X23_FULL,
    X24_FULL, X25_FULL, X26_FULL, X27_FULL, X28_FULL, X29_FULL, X30_FULL,
];

pub const CALLER_SAVED_REGISTERS: &[Arm64Register] = &[
    X0_FULL, X1_FULL, X2_FULL, X3_FULL, X4_FULL, X5_FULL, X6_FULL, X7_FULL,
    X8_FULL, X9_FULL, X10_FULL, X11_FULL, X12_FULL, X13_FULL, X14_FULL, X15_FULL,
    X16_FULL, X17_FULL,
];

pub const CALLEE_SAVED_REGISTERS: &[Arm64Register] = &[
    X19_FULL, X20_FULL, X21_FULL, X22_FULL, X23_FULL, X24_FULL, X25_FULL,
    X26_FULL, X27_FULL, X28_FULL, X29_FULL, X30_FULL,
];

pub fn get_random_general_purpose_register() -> &'static Arm64Register {
    let mut rng = rand::thread_rng();
    GENERAL_PURPOSE_REGISTERS.choose(&mut rng).unwrap()
}

pub fn get_safe_random_general_purpose_register(excludes: &[Arm64Register]) -> &'static Arm64Register {
    let mut rng = rand::thread_rng();
    let filtered: Vec<_> = GENERAL_PURPOSE_REGISTERS
        .iter()
        .filter(|reg| !excludes.contains(reg))
        .collect();

    filtered.choose(&mut rng).expect("No available registers")
}

impl Arm64Register {
    pub fn is_parameter_register(&self) -> bool {
        matches!(self.x, XRegister::X0..=XRegister::X7)
    }

    pub fn is_callee_saved(&self) -> bool {
        matches!(self.x, XRegister::X19..=XRegister::X30)
    }

    pub fn is_caller_saved(&self) -> bool {
        matches!(self.x, XRegister::X0..=XRegister::X17)
    }

    pub fn is_special_purpose(&self) -> bool {
        matches!(self.x,
            XRegister::X29 |
            XRegister::X30
        )
    }
}
