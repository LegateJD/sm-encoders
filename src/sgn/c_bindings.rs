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

use crate::core::encoder::Encoder;
use crate::sgn::encoder::SgnEncoderX64;

#[repr(C)]
pub struct CByteArray {
    pub data: *mut u8,
    pub len: usize,
    pub capacity: usize,
}

#[no_mangle]
pub extern "C" fn sgn_encoder_x64_new(
    seed: u8,
    plain_decoder: bool,
    encoding_count: u32,
    save_registers: bool
) -> *mut SgnEncoderX64 {
    let encoder = Box::new(SgnEncoderX64::builder()
        .set_seed(seed)
        .set_plain_decoder(plain_decoder)
        .set_encoding_count(encoding_count)
        .set_save_registers(save_registers)
        .build());
    Box::into_raw(encoder)
}

#[no_mangle]
pub extern "C" fn sgn_encoder_x64_free(encoder: *mut SgnEncoderX64) {
    if !encoder.is_null() {
        unsafe {
            drop(Box::from_raw(encoder));
        }
    }
}

#[no_mangle]
pub extern "C" fn sgn_encoder_x64_encode(
    encoder: *mut SgnEncoderX64,
    payload: *const u8,
    payload_len: usize,
    out: *mut CByteArray,
) -> i32 {
    if encoder.is_null() || payload.is_null() || out.is_null() {
        return -1;
    }

    unsafe {
        let encoder_ref = &mut *encoder;
        let payload_slice = std::slice::from_raw_parts(payload, payload_len);

        match encoder_ref.encode(payload_slice) {
            Ok(mut result) => {
                let len = result.len();
                let capacity = result.capacity();
                let data = result.as_mut_ptr();
                std::mem::forget(result);

                (*out).data = data;
                (*out).len = len;
                (*out).capacity = capacity;
                0
            }
            Err(_) => -2,
        }
    }
}

#[no_mangle]
pub extern "C" fn sgn_free_byte_array(array: *mut CByteArray) {
    if !array.is_null() {
        unsafe {
            let array_ref = &*array;
            if !array_ref.data.is_null() {
                drop(Vec::from_raw_parts(
                    array_ref.data,
                    array_ref.len,
                    array_ref.capacity,
                ));
            }
        }
    }
}