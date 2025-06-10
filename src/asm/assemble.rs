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

use keystone_engine::{Arch, Keystone, Mode, OptionType, OptionValue};

#[derive(Debug)]
pub enum AssemblerError {
    Configuration,
    SyntaxError
}

pub fn assemble(assembly: &str) -> Result<Vec<u8>, AssemblerError> {
    let engine = Keystone::new(Arch::X86, Mode::MODE_64).map_err(|_| AssemblerError::Configuration)?;

    engine
        .option(OptionType::SYNTAX, OptionValue::SYNTAX_INTEL)
        .map_err(|_| AssemblerError::Configuration)?;

    let result = engine.asm(assembly.to_string(), 0).map_err(|_| AssemblerError::SyntaxError)?;

    Ok(result.bytes)
}
