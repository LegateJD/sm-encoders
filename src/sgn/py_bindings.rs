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
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use std::collections::HashSet;

#[cfg(feature = "python")]
use crate::core::encoder::Encoder;
#[cfg(feature = "python")]
use crate::sgn::encoder::{
    SgnEncoderX64ChaCha as CoreSgnEncoderX64ChaCha,
    SgnEncoderX64ThreadRng as CoreSgnEncoderX64ThreadRng,
};

#[cfg(feature = "python")]
#[pyclass(unsendable)]
pub struct SgnEncoderX64ThreadRng {
    encoder: CoreSgnEncoderX64ThreadRng,
}

#[cfg(feature = "python")]
#[pymethods]
impl SgnEncoderX64ThreadRng {
    #[new]
    #[pyo3(signature = (
        plain_decoder=false,
        encoding_count=1,
        save_registers=false,
        badchars=vec![],
    ))]
    fn new(
        plain_decoder: bool,
        encoding_count: u32,
        save_registers: bool,
        badchars: Vec<u8>,
    ) -> Self {
        let badchars: HashSet<u8> = badchars.into_iter().collect();

        Self {
            encoder: CoreSgnEncoderX64ThreadRng::builder()
                .set_plain_decoder(plain_decoder)
                .set_encoding_count(encoding_count)
                .set_save_registers(save_registers)
                .set_badchars(badchars)
                .build(),
        }
    }

    fn encode(&mut self, payload: Vec<u8>) -> PyResult<Vec<u8>> {
        self.encoder
            .encode(&payload)
            .map_err(|e| PyRuntimeError::new_err(format!("Encoding error: {}", e)))
    }

    fn __repr__(&self) -> String {
        "SgnEncoderX64ThreadRng()".to_string()
    }
}

#[cfg(feature = "python")]
#[pyclass(unsendable)]
pub struct SgnEncoderX64ChaCha {
    encoder: Box<CoreSgnEncoderX64ChaCha>,
}

#[cfg(feature = "python")]
#[pymethods]
impl SgnEncoderX64ChaCha {
    #[new]
    #[pyo3(signature = (
        seed=0,
        plain_decoder=false,
        encoding_count=1,
        save_registers=false,
        badchars=vec![],
    ))]
    fn new(
        seed: u64,
        plain_decoder: bool,
        encoding_count: u32,
        save_registers: bool,
        badchars: Vec<u8>,
    ) -> Self {
        let badchars: HashSet<u8> = badchars.into_iter().collect();

        Self {
            encoder: Box::new(
                CoreSgnEncoderX64ChaCha::builder()
                    .set_plain_decoder(plain_decoder)
                    .set_encoding_count(encoding_count)
                    .set_save_registers(save_registers)
                    .set_badchars(badchars)
                    .build_with_rng_seed(seed),
            ),
        }
    }

    fn encode(&mut self, payload: Vec<u8>) -> PyResult<Vec<u8>> {
        self.encoder
            .encode(&payload)
            .map_err(|e| PyRuntimeError::new_err(format!("Encoding error: {}", e)))
    }

    fn __repr__(&self) -> String {
        "SgnEncoderX64ChaCha()".to_string()
    }
}
