mod str_test;
mod tuple_test;
mod struct_test;
mod enum_test;
mod array_test;
mod proccess_test;
mod match_test;
mod option_test;
mod pattern_test;
mod elevator;
mod method;
mod generic;
mod trait_test;
mod vector;
mod hashmap;
mod lifetime;
mod error;
use rust_helloworld::{passenger, passenger_test};
use rust_helloworld::passenger::behavior::call_elevator as call;
use rust_helloworld::doc::doc_test;

fn hello_world() {
    let chinese = "你好，世界！";
    let japanese = "こんにちは世界";
    let korean = "안녕하세요 세계";
    let regions = [chinese, japanese, korean];
    let len = regions.len();

    println!("Iterate the regions array, len = {}", len);
    for region in regions {
        println!("{}", region);
    }

    // 变量遮蔽, 改变变量的内容
    let len = 0;
    println!("Iterate over, clear regions, len = {}", len);

    // 可变字符串
    let mut hello_world = String::from("Hello, ");
    hello_world.push_str("world!");
    println!("{}", hello_world);
}

fn _int_overflow() {
    // int overflow
    let mut i: u8 = 255;
    println!("i = {}", i);
    i += 1;
    println!("i = {}", i);
}

fn int_type () {
    let twenty_six: i32 = 26;
    let one_million: i64 = 1_000_000;

    println!("twenty_six = {}", twenty_six);
    println!("one_million power 2 = {}", one_million.pow(2));

    // error: type mismatch, failed to add `i32` to `i64`
    // println!("one_million + twenty_six = {}", one_million + twenty_six);

    // range, 不包含最后一个元素
    let range = 0..10;
    println!("range = {:?}", range);
    for i in range {
        print!("{} ", i);
    }
    println!();

    // 包含最后一个元素
    for i in 0..=5 {
        print!("{} ", i);
    }
    println!();

    // 字符序列
    for i in 'a'..='g' {
        print!("{} ", i);
    }
    println!();

    // int_overflow();
}


fn char_type() {
    let c = 'c';
    let z = '字';
    let e = '🦀';

    println!("字符长度: {} {} {}", c.len_utf8(), z.len_utf8(), e.len_utf8());
    println!("字符变量在内存中的大小: {} {} {}", std::mem::size_of_val(&c), std::mem::size_of_val(&z), std::mem::size_of_val(&e));
}


fn copy() {
    // 深拷贝, 会复制堆上的数据
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("deep copy: s1 = {}, s2 = {}", s1, s2);

    // 在栈上的数据, 会复制一份
    let s3 = 100;
    let s4 = s3;
    println!("stack data copy: s3 = {}, s4 = {}", s3, s4);
}

fn changedable_ref() {
    let s = String::from("unchanged");

    fn test_unchangedable_inside(s: &String) {
        // error: cannot assign to `*s` because it is borrowed
        // *s = String::from("changed");
        println!("unchangedable inside: {}", s)
    }

    fn test_changedable_inside(s: &mut String) {
        *s = String::from("changed");
        println!("changedable inside: {}", s)
    }

    println!("changedable outside: {}", s);
    test_unchangedable_inside(&s);
    println!("unchangedable outside: {}", s);

    let mut s = String::from("unchanged");
    println!("changedable outside: {}", s);
    test_changedable_inside(&mut s);
    println!("changedable outside: {}", s);
}


fn repeat_use_var() {
    let mut s = String::from("hello");

    let s1 = &s;
    let s2 = &s;
    println!("s1 = {}, s2 = {}", s1, s2);
    println!("s1 = {}, s2 = {}", s1, s2);

    // 变量不可变引用s1, s2的作用域结束后, 可变引用s3的作用域开始, 不会产生冲突
    let s3 = &mut s;
    println!("s3 = {}", s3);
}


fn main() {
    hello_world();
    int_type();
    char_type();
    copy();
    changedable_ref();
    repeat_use_var();

    str_test::slice();
    str_test::str_op();

    tuple_test::tuple_learning();

    struct_test::struct_test();

    enum_test::enum_test();

    array_test::array_test();

    proccess_test::process_test();

    match_test::match_test();
    option_test::option_test();
    pattern_test::pattern_test();

    method::method_test();

    generic::generic();

    trait_test::trait_test();

    vector::vector();
    hashmap::hashmap();

    lifetime::lifetime();

    error::error();

    // test lib import
    passenger::behavior::call_elevator();
    passenger_test();
    call();

    doc_test();
}