#ifndef SGN_ENCODER_H
#define SGN_ENCODER_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#include "ranis_buffer.h"

typedef struct ranis_sgn_encoder_x64_chacha_t ranis_sgn_encoder_x64_chacha_t;
typedef struct ranis_sgn_encoder_x64_thread_rng_t ranis_sgn_encoder_x64_thread_rng_t;
typedef struct ranis_sgn_encoder_x32_t ranis_sgn_encoder_x32_t;
typedef struct ranis_sgn_encoder_aarch64_t ranis_sgn_encoder_aarch64_t;

// X64 SGN ChaCha Encoder
ranis_sgn_encoder_x64_chacha_t* ranis_sgn_encoder_x64_chacha_new(uint64_t seed, bool plain_decoder, uint32_t encoding_count, bool save_registers, const uint8_t* badchars, size_t badchars_len);
void ranis_sgn_encoder_x64_chacha_free(ranis_sgn_encoder_x64_chacha_t* encoder);
int32_t ranis_sgn_encoder_x64_chacha_encode(ranis_sgn_encoder_x64_chacha_t* encoder, const uint8_t* payload, size_t payload_len, ranis_buffer_t* out);

// X64 SGN ThreadRng Encoder
ranis_sgn_encoder_x64_thread_rng_t* ranis_sgn_encoder_x64_thread_new(bool plain_decoder, uint32_t encoding_count, bool save_registers, const uint8_t* badchars, size_t badchars_len);
void ranis_sgn_encoder_x64_thread_free(ranis_sgn_encoder_x64_thread_rng_t* encoder);
int32_t ranis_sgn_encoder_x64_thread_encode(ranis_sgn_encoder_x64_thread_rng_t* encoder, const uint8_t* payload, size_t payload_len, ranis_buffer_t* out);

#endif // SGN_ENCODER_H