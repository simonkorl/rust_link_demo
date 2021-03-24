#ifndef __OUTER_H__
#define __OUTER_H__
#include <stdlib.h>
// 在这个文件中声明所有在动态链接库中的函数
extern "C" {
    // 将 Rust 的 struct 转换为 C 的形式
    struct MyComplex{
        double x;
        double y;
    };

    double ComplexMod(const MyComplex* complex);
    size_t c_interface_1(size_t a);
    bool c_interface_2();
    void c_interface_3();
}
#endif // __OUTER_H__