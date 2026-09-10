#ifndef RANIS_BUFFER_H
#define RANIS_BUFFER_H

#include <stddef.h>
#include <stdint.h>

typedef struct {
    uint8_t *data;
    size_t len;
    size_t capacity;
} ranis_buffer_t;

// Free a byte array previously populated by one of the `*_encode` functions.
void ranis_free_buffer(ranis_buffer_t* array);

#endif // RANIS_BUFFER_H
