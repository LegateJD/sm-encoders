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
use pyo3::exceptions::PyRuntimeError;

#[cfg(feature = "python")]
use crate::core::encoder::Encoder;
#[cfg(feature = "python")]
use crate::sgn::encoder::{SgnEncoderX64, SgnEncoderX32, SgnEncoderAArch64};

#[cfg(feature = "python")]
#[pyclass]
pub struct PySgnEncoderX64 {
    encoder: SgnEncoderX64,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySgnEncoderX64 {
    #[new]
    #[pyo3(signature = (seed=0, plain_decoder=false))]
    fn new(seed: u8, plain_decoder: bool) -> Self {
        Self {
            encoder: SgnEncoderX64::new(seed, plain_decoder),
        }
    }

    fn encode(&self, payload: Vec<u8>) -> PyResult<Vec<u8>> {
        self.encoder
            .encode(&payload)
            .map_err(|e| PyRuntimeError::new_err(format!("Encoding error: {}", e)))
    }

    fn __repr__(&self) -> String {
        "SgnEncoderX64()".to_string()
    }
}

#[cfg(feature = "python")]
#[pyclass]
pub struct PySgnEncoderX32 {
    encoder: SgnEncoderX32,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySgnEncoderX32 {
    #[new]
    #[pyo3(signature = (seed=0, plain_decoder=false))]
    fn new(seed: u8, plain_decoder: bool) -> Self {
        Self {
            encoder: SgnEncoderX32::new(seed, plain_decoder),
        }
    }

    fn encode(&self, payload: Vec<u8>) -> PyResult<Vec<u8>> {
        self.encoder
            .encode(&payload)
            .map_err(|e| PyRuntimeError::new_err(format!("Encoding error: {}", e)))
    }

    fn __repr__(&self) -> String {
        "SgnEncoderX32()".to_string()
    }
}

#[cfg(feature = "python")]
#[pyclass]
pub struct PySgnEncoderAArch64 {
    encoder: SgnEncoderAArch64,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySgnEncoderAArch64 {
    #[new]
    #[pyo3(signature = (seed=0, plain_decoder=false))]
    fn new(seed: u8, plain_decoder: bool) -> Self {
        Self {
            encoder: SgnEncoderAArch64::new(seed, plain_decoder),
        }
    }

    fn encode(&self, payload: Vec<u8>) -> PyResult<Vec<u8>> {
        self.encoder
            .encode(&payload)
            .map_err(|e| PyRuntimeError::new_err(format!("Encoding error: {}", e)))
    }

    fn __repr__(&self) -> String {
        "SgnEncoderAArch64()".to_string()
    }
}