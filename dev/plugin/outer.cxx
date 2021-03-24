#include <outer.hxx>
#include <math.h>
#include <stdio.h>

double ComplexMod(const MyComplex* complex) {
    return sqrt(complex->x * complex->x + complex->y * complex->y);
}

size_t c_interface_1(size_t a) {
    printf("I'm in c interface 1!\n");
    return 9;
}

bool c_interface_2() {
    printf("I'm returning a meaningless 'true'\n");
    return true;
}

void c_interface_3() {
    printf("I should have done something here, but I'm tired. zzz.\n");
}