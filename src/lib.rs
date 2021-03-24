// 下面的结构体可以在 C 中直接使用
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MyComplex{
    x: f64,
    y: f64
}

// 以下展示出如何在 Rust 中写出一个可以给 C 使用的函数
#[no_mangle] // ! 这一行必须要写
pub extern "C" fn lib_main() {
    let c = MyComplex {x : 3.0, y: 4.0};
    unsafe {
        println!("Myfunc({:?}) = {}", c, ComplexFunc(&c));
    }
}

extern "C"{
    pub fn ComplexFunc(c: *const MyComplex) -> f64;
}

// 使用以下两行代码使用 C 的数据类型，其他的数据类型请搜索 libc 的文档
extern crate libc;
use libc::size_t;

pub fn do_1() {
    println!("I'm doing task 1 ...");
}
pub fn do_2() {
    println!("I'm doing task 2 ...");
}
pub fn do_3() {
    println!("I'm doing task 3 ...");
    r_interface_3(); // 修改原函数的作用
}

extern "C" {
    pub fn c_interface_1 (a: size_t) -> size_t;
    pub fn c_interface_2 () -> bool;
    pub fn c_interface_3 ();
}
pub fn r_interface_1(a: size_t) -> size_t {
    // C 的函数无法保证安全性。建议嵌套一层，防止安全性检测出现问题
    unsafe {
        c_interface_1 (a)
    }
}
pub fn r_interface_2() -> bool {
    unsafe {
        c_interface_2 ()
    }
}
pub fn r_interface_3() {
    unsafe {
        c_interface_3();
    }
}
#[no_mangle]
pub fn do_something() {
    r_interface_1(0); // 向源代码中插入函数
    do_1();
    // do_2();
    r_interface_2(); // 替换原函数
    do_3();
}