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

pub fn assemble(assembly: &str) -> Result<Vec<u8>, u8> {
    let engine = Keystone::new(Arch::X86, Mode::MODE_64);

    match engine {
        Ok(en) => {
            en.option(OptionType::SYNTAX, OptionValue::SYNTAX_INTEL);

            let result = en.asm(assembly.to_string(), 0);

            match result {
                Ok(r) => return Ok(r.bytes),
                Err(_) => return Err(3),
            }
        }
        Err(_) => return Err(2),
    }
}
