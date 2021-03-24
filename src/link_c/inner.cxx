#include <inner.hxx>
#include <math.h>

double ComplexFunc(const MyComplex * complex) {
    struct MyComplex tmp = {
        .x = 10,
        .y = 20,
    };

    tmp.x += complex->x;
    tmp.y += complex->y;

    return ComplexMod(&tmp);
}