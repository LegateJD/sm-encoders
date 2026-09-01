#include <stdio.h>
#include "sgn_encoder.h"

// mov    eax,0x42
// ret
unsigned char shellcode[] = {
    0xb8, 0x42, 0x00, 0x00, 0x00, 0xc3
};

int main() {
   SgnEncoderX64ChaCha* encoder = sgn_encoder_x64_chacha_new(54, false, 1, false, false, NULL, 0);

   if (encoder == NULL) {
       fprintf(stderr, "Failed to create encoder\n");
       return 1;
   }

   CByteArray out = { .data = NULL, .len = 0, .capacity = 0 };
   printf("Encoding!\n");
   int result = sgn_encoder_x64_chacha_encode(encoder, shellcode, sizeof(shellcode), &out);

   if (result != 0) {
       fprintf(stderr, "Failed to encode payload \n");
       return 1;
   }

   uint8_t *data = out.data;
   size_t len = out.len;

   printf("Payload:\n");

   for (size_t i = 0; i < len; i++) {
       printf("0x%02x%s", data[i], (i + 1 < len) ? ", " : "\n");
   }

   sgn_encoder_x64_chacha_free(encoder);

   return 0;
}
