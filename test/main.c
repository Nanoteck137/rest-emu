#include <stdint.h>

uint32_t _start(uint32_t a, uint32_t b) {
    if (a == b) {
        return -1;
    } else if (a > b) {
        return -2;
    } else {
        return -3;
    }
}
