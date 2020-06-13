#include <stdio.h>

#include "rust_lib.h"

int main() {

    rust_function();

    printf("%d", rust_acc_fnc(5, 10));

    return 0;
}