#include <stdint.h>

int _start(int number1, int number2) {
    if(number1 == number2) {
        return 1;
    }
    else if (number1 > number2) {
        return 2;
    }
    else {
        return 3;
    }

    return 0;
}
