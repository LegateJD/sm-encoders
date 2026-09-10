#include <stdint.h>
#include <stdio.h>
#include "ranis.h"
#include <stddef.h>

// mov    eax,0x42
// ret
unsigned char shellcode[] = {
    0xb8, 0x42, 0x00, 0x00, 0x00, 0xc3
};

int main() {
    ranis_sgn_encoder_x64_chacha_t* encoder = ranis_sgn_encoder_x64_chacha_new(54, false, 1, false, NULL, 0);

    if (encoder == NULL) {
        fprintf(stderr, "Failed to create encoder\n");
        return 1;
    }

    ranis_buffer_t out = { .data = NULL, .len = 0, .capacity = 0 };
    printf("Encoding!\n");
    int32_t result = ranis_sgn_encoder_x64_chacha_encode(encoder, shellcode, sizeof(shellcode), &out);
    ranis_sgn_encoder_x64_chacha_free(encoder);

    if (result != 0) {
        fprintf(stderr, "Failed to encode payload\n");
        return 1;
    }

    printf("Payload:\n");

    for (size_t i = 0; i < out.len; i++) {
        printf("0x%02x%s", out.data[i], (i + 1 < out.len) ? ", " : "\n");
    }

    ranis_free_buffer(&out);

    return 0;
}
