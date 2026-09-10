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
    sgn::encoder::{SgnEncoderX64ChaCha, SgnEncoderX64ThreadRng},
};

#[repr(C)]
pub struct Buffer {
    pub data: *mut u8,
    pub len: usize,
    pub capacity: usize,
}

/// # Safety
/// `badchars` must be either null or valid for reads of `badchars_len` bytes.
#[no_mangle]
pub unsafe extern "C" fn ranis_sgn_encoder_x64_chacha_new(
    seed: u64,
    plain_decoder: bool,
    encoding_count: u32,
    save_registers: bool,
    badchars: *const u8,
    badchars_len: usize,
) -> *mut SgnEncoderX64ChaCha {
    let mut encoder_builder = SgnEncoderX64ChaCha::builder()
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

/// # Safety
/// `encoder` must be a pointer previously returned by `ranis_sgn_encoder_x64_chacha_new`, or null.
#[no_mangle]
pub unsafe extern "C" fn ranis_sgn_encoder_x64_chacha_free(encoder: *mut SgnEncoderX64ChaCha) {
    if !encoder.is_null() {
        unsafe {
            drop(Box::from_raw(encoder));
        }
    }
}

/// # Safety
/// `encoder` must be a valid pointer from `ranis_sgn_encoder_x64_thread_new`, `payload` must be valid
/// for reads of `payload_len` bytes, and `out` must be a valid pointer to a `Buffer`.
/// # Safety
/// `encoder` must be a valid pointer from `ranis_sgn_encoder_x64_chacha_new`, `payload` must be valid
/// for reads of `payload_len` bytes, and `out` must be a valid pointer to a `Buffer`.
#[no_mangle]
pub unsafe extern "C" fn ranis_sgn_encoder_x64_chacha_encode(
    encoder: *mut SgnEncoderX64ChaCha,
    payload: *const u8,
    payload_len: usize,
    out: *mut Buffer,
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

/// # Safety
/// `array` must be a pointer to a `Buffer` previously populated by one of the `*_encode`
/// functions in this module, or null.
#[no_mangle]
pub unsafe extern "C" fn ranis_free_buffer(array: *mut Buffer) {
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

/// # Safety
/// `badchars` must be either null or valid for reads of `badchars_len` bytes.
#[no_mangle]
pub unsafe extern "C" fn ranis_sgn_encoder_x64_thread_new(
    plain_decoder: bool,
    encoding_count: u32,
    save_registers: bool,
    badchars: *const u8,
    badchars_len: usize,
) -> *mut SgnEncoderX64ThreadRng {
    let mut encoder_builder = SgnEncoderX64ThreadRng::builder()
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

/// # Safety
/// `encoder` must be a pointer previously returned by `ranis_sgn_encoder_x64_thread_new`, or null.
#[no_mangle]
pub unsafe extern "C" fn ranis_sgn_encoder_x64_thread_free(encoder: *mut SgnEncoderX64ThreadRng) {
    if !encoder.is_null() {
        unsafe {
            drop(Box::from_raw(encoder));
        }
    }
}

/// # Safety
/// `encoder` must be a valid pointer from `ranis_sgn_encoder_x64_thread_new`, `payload` must be valid
/// for reads of `payload_len` bytes, and `out` must be a valid pointer to a `CByteArray`.
#[no_mangle]
pub unsafe extern "C" fn ranis_sgn_encoder_x64_thread_encode(
    encoder: *mut SgnEncoderX64ThreadRng,
    payload: *const u8,
    payload_len: usize,
    out: *mut Buffer,
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
