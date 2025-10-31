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

typedef struct SgnEncoderX64 SgnEncoderX64;
typedef struct SgnEncoderX32 SgnEncoderX32;
typedef struct SgnEncoderAArch64 SgnEncoderAArch64;

// X64 Encoder
SgnEncoderX64* sgn_encoder_x64_new(uint8_t seed, bool plain_decoder, uint32_t encoding_count, bool save_registers);
void sgn_encoder_x64_free(SgnEncoderX64* encoder);
int32_t sgn_encoder_x64_encode(const SgnEncoderX64* encoder, const uint8_t* payload, size_t payload_len, CByteArray* out);

// Free the byte array returned by encode
void sgn_free_byte_array(CByteArray* array);

#endif // SGN_ENCODER_H