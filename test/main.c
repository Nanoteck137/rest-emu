#include <stdint.h>

int _start(uint32_t number1, uint32_t number2) {
    // asm("rdtime x1");
    asm("li t1, 0b1\n\t"
        "csrrci t0, 0xfff, 0b001");

    if (number1 >= number2) {
        return 2;
    }

    return 0;
}
