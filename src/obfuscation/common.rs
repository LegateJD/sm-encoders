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

pub trait GarbageJump {
    fn add_jmp_over(&self, payload: &[u8]) -> Vec<u8>;

    fn generate_garbage_jump(&self) -> Vec<u8>;
}

pub trait GarbageAssembly {
    fn generate_garbage_assembly(&self) -> Vec<u8>;
}

pub trait CallOver {
    fn add_call_over(&self, payload: Vec<u8>) -> Vec<u8>;
}

pub trait GarbageInstructions {
    fn generate_garbage_instructions(&self) -> Vec<u8>;
}
