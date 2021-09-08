#include <stdint.h>

int32_t _start() {
    int32_t a = 0b10000;
    int32_t b = 3;
    int32_t c = a >> b;

    return c;
}
