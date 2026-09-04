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

use crate::{
    core::encoder::Encoder,
    sgn::c_bindings::CByteArray,
    xor_dynamic::encoder::{XorDynamicEncoderX64ChaCha, XorDynamicEncoderX64Thread},
};

#[no_mangle]
pub extern "C" fn xor_dynamic_encoder_x64_chacha_new(
    seed: u64,
    plain_decoder: bool,
    encoding_count: u32,
    save_registers: bool,
    badchars: *const u8,
    badchars_len: usize,
) -> *mut XorDynamicEncoderX64ChaCha {
    let mut encoder_builder = XorDynamicEncoderX64ChaCha::builder()
        .set_plain_decoder(plain_decoder)
        .set_encoding_count(encoding_count)
        .set_save_registers(save_registers);

    if !badchars.is_null() && badchars_len > 0 {
        let badchars_slice = unsafe { std::slice::from_raw_parts(badchars, badchars_len) };
        let badchars: std::collections::HashSet<u8> = badchars_slice.iter().copied().collect();
        encoder_builder = encoder_builder.set_badchars(badchars);
    }

    let encoder = Box::new(encoder_builder.build_with_rng_seed(seed));
    Box::into_raw(encoder)
}

#[no_mangle]
pub extern "C" fn xor_dynamic_encoder_x64_chacha_free(encoder: *mut XorDynamicEncoderX64ChaCha) {
    if !encoder.is_null() {
        unsafe {
            drop(Box::from_raw(encoder));
        }
    }
}

#[no_mangle]
pub extern "C" fn xor_dynamic_encoder_x64_chacha_encode(
    encoder: *mut XorDynamicEncoderX64ChaCha,
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
pub extern "C" fn xor_dynamic_encoder_x64_thread_new(
    plain_decoder: bool,
    encoding_count: u32,
    save_registers: bool,
    badchars: *const u8,
    badchars_len: usize,
) -> *mut XorDynamicEncoderX64Thread {
    let mut encoder_builder = XorDynamicEncoderX64Thread::builder()
        .set_plain_decoder(plain_decoder)
        .set_encoding_count(encoding_count)
        .set_save_registers(save_registers);

    if !badchars.is_null() && badchars_len > 0 {
        let badchars_slice = unsafe { std::slice::from_raw_parts(badchars, badchars_len) };
        let badchars: std::collections::HashSet<u8> = badchars_slice.iter().copied().collect();
        encoder_builder = encoder_builder.set_badchars(badchars);
    }

    let encoder = Box::new(encoder_builder.build());
    Box::into_raw(encoder)
}

#[no_mangle]
pub extern "C" fn xor_dynamic_encoder_x64_thread_free(encoder: *mut XorDynamicEncoderX64Thread) {
    if !encoder.is_null() {
        unsafe {
            drop(Box::from_raw(encoder));
        }
    }
}

#[no_mangle]
pub extern "C" fn xor_dynamic_encoder_x64_thread_encode(
    encoder: *mut XorDynamicEncoderX64Thread,
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
