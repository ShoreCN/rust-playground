use std::slice::from_raw_parts;
use std::str::from_utf8;

fn raw_pointer() {
    let mut x = 1;
    let const_raw_ptr = &x as *const i32;
    let mut_raw_ptr = &mut x as *mut i32;

    unsafe {
        println!("const_raw_ptr = {}", *const_raw_ptr);
        *mut_raw_ptr = 2;
        println!("mut_raw_ptr = {}", *mut_raw_ptr);
        println!("const_raw_ptr = {}", *const_raw_ptr);
    }

    // 通过裸指针加上长度, 可以访问内存中的一段str数据
    let s = "This string is to test raw pointer";
    let s_raw_ptr = s.as_ptr();
    let s_len = s.len();
    unsafe {
        // print the content of s_ptr
        let s_ptr = from_raw_parts(s_raw_ptr, s_len);
        println!("s_ptr = {:?}", s_ptr);
        // print the content of s_ptr as string
        let s_str = from_utf8(s_ptr).unwrap();
        println!("s_str = {}", s_str);
    }

    // 也可以通过str的引用获取str的裸指针, 直接解引用就可以得到str
    let s = "This string is to test raw pointer";
    let s_raw_ptr = &s as *const &str;
    unsafe {
        println!("s_raw_ptr = {}", *s_raw_ptr);
    }

    // 创建智能指针的裸指针
    let box_x = Box::new(666);
    let direct_raw_ptr = &*box_x as *const i32;
    let into_raw_ptr = Box::into_raw(box_x);
    unsafe {
        println!("direct_raw_ptr = {}", *direct_raw_ptr);
        println!("into_raw_ptr = {}", *into_raw_ptr);
    }
}

unsafe fn unsafe_func() {
    println!("This is an unsafe function");
}

// 通过FFI(外部函数接口)调用C语言的函数
extern "C" {
    fn abs(input: i32) -> i32;
}

fn call_ffi() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

pub fn unsafe_code() {
    raw_pointer();
    unsafe {
        unsafe_func();
    }
    call_ffi();
}