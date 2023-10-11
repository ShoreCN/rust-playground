use std::net::IpAddr;

fn trigger_panic() {
    // 代码中出现panic
    // let arr = [1, 2, 3];
    // println!("arr[4] = {}", arr[4]);

    // 主动触发panic
    // panic!("trigger panic");
}

fn handle_panic() {
    // parse方法返回Result类型, 如果解析失败, 则返回Err(E), 如果解析成功, 则返回Ok(T)
    // unwrap方法会自动处理Result类型, 如果是Err(E), 则会触发panic, 如果是Ok(T), 则会返回T
    let ip: IpAddr = "0.0.0.1".parse().unwrap();
    println!("ip = {}", ip);

    // expect的作用: 如果解析失败, 则会触发panic, 并且可以自定义panic的提示信息
    // 提示信息通常用于解释panic的原因
    let ip: IpAddr = "999.1.1.1".parse().expect("parse ip failed");
    println!("ip = {}", ip);
}


pub fn error() {
    trigger_panic();
    handle_panic();
}