#ifndef SGN_ENCODER_H
#define SGN_ENCODER_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

typedef struct {
    uint8_t *data;
    size_t len;
    size_t capacity;
} CByteArray;

typedef struct SgnEncoderX64ChaCha SgnEncoderX64ChaCha;
typedef struct SgnEncoderX64ThreadRng SgnEncoderX64ThreadRng;
typedef struct SgnEncoderX32 SgnEncoderX32;
typedef struct SgnEncoderAArch64 SgnEncoderAArch64;

// X64 SGN ChaCha Encoder
SgnEncoderX64ChaCha* sgn_encoder_x64_chacha_new(uint64_t seed, bool plain_decoder, uint32_t encoding_count, bool save_registers, const uint8_t* badchars, size_t badchars_len);
void sgn_encoder_x64_chacha_free(SgnEncoderX64ChaCha* encoder);
int32_t sgn_encoder_x64_chacha_encode(SgnEncoderX64ChaCha* encoder, const uint8_t* payload, size_t payload_len, CByteArray* out);

// X64 SGN ThreadRng Encoder
SgnEncoderX64ThreadRng* sgn_encoder_x64_thread_new(bool plain_decoder, uint32_t encoding_count, bool save_registers, const uint8_t* badchars, size_t badchars_len);
void sgn_encoder_x64_thread_free(SgnEncoderX64ThreadRng* encoder);
int32_t sgn_encoder_x64_thread_encode(SgnEncoderX64ThreadRng* encoder, const uint8_t* payload, size_t payload_len, CByteArray* out);

// X32 SGN Encoder
SgnEncoderX32* sgn_encoder_x32_new(bool plain_decoder, uint32_t encoding_count, bool save_registers, const uint8_t* badchars, size_t badchars_len);
void sgn_encoder_x32_free(SgnEncoderX32* encoder);
int32_t sgn_encoder_x32_encode(SgnEncoderX32* encoder, const uint8_t* payload, size_t payload_len, CByteArray* out);

// Free the byte array returned by encode
void sgn_free_byte_array(CByteArray* array);

#endif // SGN_ENCODER_H