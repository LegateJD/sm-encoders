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

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

use crate::core::encoder::Encoder;
use crate::xor_dynamic::encoder::{
    XorDynamicEncoderError, XorDynamicEncoderX64ChaCha, XorDynamicEncoderX64Thread,
};

enum XorDynamicEncoderX64Inner {
    ChaCha(XorDynamicEncoderX64ChaCha),
    Thread(XorDynamicEncoderX64Thread),
}

impl XorDynamicEncoderX64Inner {
    fn encode(&mut self, payload: &[u8]) -> Result<Vec<u8>, XorDynamicEncoderError> {
        match self {
            XorDynamicEncoderX64Inner::ChaCha(encoder) => encoder.encode(payload),
            XorDynamicEncoderX64Inner::Thread(encoder) => encoder.encode(payload),
        }
    }
}

#[pyclass(unsendable)]
pub struct PyXorDynamicEncoderX64 {
    encoder: XorDynamicEncoderX64Inner,
}

#[pymethods]
impl PyXorDynamicEncoderX64 {
    /// `rng` selects the assembler's RNG source: "chacha" (seeded, deterministic)
    /// or "thread" (OS RNG, ignores `seed`).
    #[new]
    #[pyo3(signature = (
        seed=0,
        plain_decoder=false,
        encoding_count=1,
        save_registers=false,
        badchars=vec![],
        ascii_printable=false,
        rng="thread"
    ))]
    fn new(
        seed: u64,
        plain_decoder: bool,
        encoding_count: u32,
        save_registers: bool,
        badchars: Vec<u8>,
        ascii_printable: bool,
        rng: &str,
    ) -> PyResult<Self> {
        let badchars: HashSet<u8> = badchars.into_iter().collect();

        let encoder = match rng {
            "chacha" => XorDynamicEncoderX64Inner::ChaCha(
                XorDynamicEncoderX64ChaCha::builder()
                    .set_plain_decoder(plain_decoder)
                    .set_encoding_count(encoding_count)
                    .set_save_registers(save_registers)
                    .set_badchars(badchars)
                    .set_ascii_printable(ascii_printable)
                    .build_with_rng_seed(seed),
            ),
            "thread" => XorDynamicEncoderX64Inner::Thread(
                XorDynamicEncoderX64Thread::builder()
                    .set_plain_decoder(plain_decoder)
                    .set_encoding_count(encoding_count)
                    .set_save_registers(save_registers)
                    .set_badchars(badchars)
                    .set_ascii_printable(ascii_printable)
                    .build(),
            ),
            other => {
                return Err(PyValueError::new_err(format!(
                    "Invalid rng '{}': expected 'chacha' or 'thread'",
                    other
                )))
            }
        };

        Ok(Self { encoder })
    }

    fn encode(&mut self, payload: Vec<u8>) -> PyResult<Vec<u8>> {
        self.encoder
            .encode(&payload)
            .map_err(|e| PyRuntimeError::new_err(format!("Encoding error: {}", e)))
    }

    fn __repr__(&self) -> String {
        "XorDynamicEncoderX64()".to_string()
    }
}
