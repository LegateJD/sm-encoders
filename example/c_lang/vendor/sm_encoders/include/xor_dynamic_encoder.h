#ifndef XOR_DYNAMIC_ENCODER_H
#define XOR_DYNAMIC_ENCODER_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// Reuses CByteArray and sgn_free_byte_array for the returned encoded buffer.
#include "sgn_encoder.h"

typedef struct XorDynamicEncoderX64ChaCha XorDynamicEncoderX64ChaCha;
typedef struct XorDynamicEncoderX64Thread XorDynamicEncoderX64Thread;

// X64 XOR Dynamic ChaCha Encoder
XorDynamicEncoderX64ChaCha* xor_dynamic_encoder_x64_chacha_new(uint64_t seed, bool plain_decoder, uint32_t encoding_count, bool save_registers, bool ascii_printable, const uint8_t* badchars, size_t badchars_len);
void xor_dynamic_encoder_x64_chacha_free(XorDynamicEncoderX64ChaCha* encoder);
int32_t xor_dynamic_encoder_x64_chacha_encode(XorDynamicEncoderX64ChaCha* encoder, const uint8_t* payload, size_t payload_len, CByteArray* out);

// X64 XOR Dynamic ThreadRng Encoder
XorDynamicEncoderX64Thread* xor_dynamic_encoder_x64_thread_new(bool plain_decoder, uint32_t encoding_count, bool save_registers, bool ascii_printable, const uint8_t* badchars, size_t badchars_len);
void xor_dynamic_encoder_x64_thread_free(XorDynamicEncoderX64Thread* encoder);
int32_t xor_dynamic_encoder_x64_thread_encode(XorDynamicEncoderX64Thread* encoder, const uint8_t* payload, size_t payload_len, CByteArray* out);

#endif // XOR_DYNAMIC_ENCODER_H
