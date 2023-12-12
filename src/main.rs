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
use rust_helloworld::passenger_test;
use rust_helloworld::passenger::behavior::call_elevator as call;
use rust_helloworld::doc::doc_test;
use rust_helloworld::output::output;
mod closure;
mod iterator;
mod transfer;
mod data_type;
mod smart_pointer;
mod threads;
mod global;
mod unsafe_code;
mod async_code;

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
    // 执行函数和名称的对应表
    let funcs: Vec<(&str, fn())>  = vec![
        ("hello", hello_world),
        ("int_type", int_type),
        ("char_type", char_type),
        ("copy", copy),
        ("changedable_ref", changedable_ref),
        ("repeat_use_var", repeat_use_var),
        ("slice", str_test::slice),
        ("str_op", str_test::str_op),
        ("tuple_learning", tuple_test::tuple_learning),
        ("struct_test", struct_test::struct_test),
        ("enum_test", enum_test::enum_test),
        ("array_test", array_test::array_test),
        ("process_test", proccess_test::process_test),
        ("match_test", match_test::match_test),
        ("option_test", option_test::option_test),
        ("pattern_test", pattern_test::pattern_test),
        ("method_test", method::method_test),
        ("generic", generic::generic),
        ("trait_test", trait_test::trait_test),
        ("vector", vector::vector),
        ("hashmap", hashmap::hashmap),
        ("lifetime", lifetime::lifetime),
        ("error", error::error),
        ("passenger_test", passenger_test),
        ("call", call),
        ("doc_test", doc_test),
        ("output", output),
        ("closure", closure::closure),
        ("iterator", iterator::iterator),
        ("transfer", transfer::transfer),
        ("data_type", data_type::data_type),
        ("smart_pointer", smart_pointer::smart_pointer),
        ("threads", threads::threads),
        ("global", global::global),
        ("unsafe_code", unsafe_code::unsafe_code),
        ("async_code", async_code::async_code),
    ];

    // 根据命令行参数, 执行不同的函数, 命令行参数是可以自动推导的
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let func_name = &args[1];
        if let Some(func) = funcs.iter().find(|(name, _)| *name == func_name.as_str()) {
            println!("\n**[{}]**", func_name);
            (func.1)();
        } else {
            println!("not found function: {}", func_name);
        }
        return;
    }

    // 如果没有指定命令行参数, 则执行所有的函数
    for (name, func) in funcs {
        println!("\n**[{}]**", name);
        func();
    }
}