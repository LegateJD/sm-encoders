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

pub mod core;
pub mod obfuscation;
pub mod pipeline;
pub mod schema;
pub mod sgn;
pub mod utils;
pub mod x64_arch;
pub mod xor_dynamic;
pub mod arm64;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn sm_encoders(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<sgn::py_bindings::PySgnEncoderX64>()?;
    Ok(())
}