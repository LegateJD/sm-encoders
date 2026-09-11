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

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::core::encoder::Encoder;
use crate::xor_dynamic::encoder::{
    XorDynamicEncoderX64ChaCha as CoreXorDynamicEncoderX64ChaCha,
    XorDynamicEncoderX64Thread as CoreXorDynamicEncoderX64Thread,
};

#[pyclass(unsendable)]
pub struct XorDynamicEncoderX64ThreadRng {
    encoder: CoreXorDynamicEncoderX64Thread,
}

#[pymethods]
impl XorDynamicEncoderX64ThreadRng {
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
            encoder: CoreXorDynamicEncoderX64Thread::builder()
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
        "XorDynamicEncoderX64ThreadRng()".to_string()
    }
}

#[pyclass(unsendable)]
pub struct XorDynamicEncoderX64ChaCha {
    encoder: Box<CoreXorDynamicEncoderX64ChaCha>,
}

#[pymethods]
impl XorDynamicEncoderX64ChaCha {
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
                CoreXorDynamicEncoderX64ChaCha::builder()
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
        "XorDynamicEncoderX64ChaCha()".to_string()
    }
}
