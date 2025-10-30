use crate::core::encoder::Encoder;
use crate::sgn::encoder::{SgnEncoderX32, SgnEncoderX64};

#[repr(C)]
pub struct CByteArray {
    pub data: *mut u8,
    pub len: usize,
    pub capacity: usize,
}

#[no_mangle]
pub extern "C" fn sgn_encoder_x64_new(seed: u8, plain_decoder: bool) -> *mut SgnEncoderX64 {
    let encoder = Box::new(SgnEncoderX64::new(seed, plain_decoder));
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
    encoder: *const SgnEncoderX64,
    payload: *const u8,
    payload_len: usize,
    out: *mut CByteArray,
) -> i32 {
    if encoder.is_null() || payload.is_null() || out.is_null() {
        return -1;
    }

    unsafe {
        let encoder_ref = &*encoder;
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