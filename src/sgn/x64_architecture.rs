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

#[derive(Debug, Clone, Copy)]
pub struct Register {
    pub full: &'static str,
    pub extended: &'static str,
    pub high: &'static str,
    pub low: &'static str,
}

pub const GENERAL_PURPOSE_REGISTERS_64_BIT: &[Register] = &[
    Register {
        full: "RAX",
        extended: "EAX",
        high: "AX",
        low: "AL",
    },
    Register {
        full: "RBX",
        extended: "EBX",
        high: "BX",
        low: "BL",
    },
    Register {
        full: "RCX",
        extended: "ECX",
        high: "CX",
        low: "CL",
    },
    Register {
        full: "RDX",
        extended: "EDX",
        high: "DX",
        low: "DL",
    },
    Register {
        full: "RSI",
        extended: "ESI",
        high: "SI",
        low: "SIL",
    },
    Register {
        full: "RDI",
        extended: "EDI",
        high: "DX",
        low: "DIL",
    },
    Register {
        full: "R8",
        extended: "R8D",
        high: "R8W",
        low: "R8B",
    },
    Register {
        full: "R9",
        extended: "R9D",
        high: "R9W",
        low: "R9B",
    },
    Register {
        full: "R10",
        extended: "R10D",
        high: "R10W",
        low: "R10B",
    },
    Register {
        full: "R11",
        extended: "R11D",
        high: "R11W",
        low: "R11B",
    },
    Register {
        full: "R12",
        extended: "R12D",
        high: "R12W",
        low: "R12B",
    },
    Register {
        full: "R13",
        extended: "R13D",
        high: "R13W",
        low: "R13B",
    },
    Register {
        full: "R14",
        extended: "R14D",
        high: "R14W",
        low: "R14B",
    },
    Register {
        full: "R15",
        extended: "R15D",
        high: "R15W",
        low: "R15B",
    },
];
