#ifndef __LINK_H__
#define __LINK_H__
// 可以在该文件中注明所有的 C 的静态编译的程序
#include "outer.hxx"
#include <stdlib.h>
extern "C" {
    double ComplexFunc(const MyComplex* complex);
}

#endif // __LINK_H__