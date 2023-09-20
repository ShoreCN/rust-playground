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

    // 包含最后一个元素
    for i in 0..=5 {
        print!("{} ", i);
    }

    // 字符序列
    for i in 'a'..='g' {
        print!("{} ", i);
    }

    // int overflow
    let mut i: u8 = 255;
    println!("i = {}", i);
    i += 1;
    println!("i = {}", i);
}

fn main() {
    hello_world();
    int_type();
}