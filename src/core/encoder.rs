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

use std::error::Error;

pub trait Encoder {
    type Error: Sized + Error;

    fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, Self::Error>;
}

pub trait DecoderStub<EncoderType>
where
    EncoderType: Encoder,
{
    type Error: Sized + Error;
    type Parameters: Sized + Copy + Clone;

    fn get_decoder_stub(&self, parameters: Self::Parameters) -> Result<Vec<u8>, Self::Error>;
}

pub trait AsmInit {
    fn new() -> Self;
}

