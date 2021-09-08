#include <stdint.h>

int _start(uint32_t number1, uint32_t number2) {
    asm("ecall");
    asm("ebreak");

    if (number1 >= number2) {
        return 2;
    }

    return 0;
}
