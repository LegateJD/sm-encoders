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

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::exceptions::{PyRuntimeError, PyValueError};

#[cfg(feature = "python")]
use std::collections::HashSet;

#[cfg(feature = "python")]
use crate::core::encoder::Encoder;
#[cfg(feature = "python")]
use crate::sgn::encoder::{SgnEncoderX64ChaCha, SgnEncoderX64ThreadRng, ShikataGaNaiError};

#[cfg(feature = "python")]
enum SgnEncoderX64Inner {
    ChaCha(SgnEncoderX64ChaCha),
    Thread(SgnEncoderX64ThreadRng),
}

#[cfg(feature = "python")]
impl SgnEncoderX64Inner {
    fn encode(&mut self, payload: &[u8]) -> Result<Vec<u8>, ShikataGaNaiError> {
        match self {
            SgnEncoderX64Inner::ChaCha(encoder) => encoder.encode(payload),
            SgnEncoderX64Inner::Thread(encoder) => encoder.encode(payload),
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(unsendable)]
pub struct SgnEncoderX64 {
    encoder: SgnEncoderX64Inner,
}

#[cfg(feature = "python")]
#[pymethods]
impl SgnEncoderX64 {
    /// `rng` selects the assembler's RNG source: "chacha" (seeded, deterministic)
    /// or "thread" (OS RNG, ignores `seed`).
    #[new]
    #[pyo3(signature = (
        seed=0,
        plain_decoder=false,
        encoding_count=1,
        save_registers=false,
        badchars=vec![],
        rng="thread"
    ))]
    fn new(
        seed: u64,
        plain_decoder: bool,
        encoding_count: u32,
        save_registers: bool,
        badchars: Vec<u8>,
        rng: &str,
    ) -> PyResult<Self> {
        let badchars: HashSet<u8> = badchars.into_iter().collect();

        let encoder = match rng {
            "chacha" => SgnEncoderX64Inner::ChaCha(
                SgnEncoderX64ChaCha::builder()
                    .set_plain_decoder(plain_decoder)
                    .set_encoding_count(encoding_count)
                    .set_save_registers(save_registers)
                    .set_badchars(badchars)
                    .build_with_rng_seed(seed),
            ),
            "thread" => SgnEncoderX64Inner::Thread(
                SgnEncoderX64ThreadRng::builder()
                    .set_plain_decoder(plain_decoder)
                    .set_encoding_count(encoding_count)
                    .set_save_registers(save_registers)
                    .set_badchars(badchars)
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
        "SgnEncoderX64()".to_string()
    }
}
