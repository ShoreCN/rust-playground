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
    let ip: IpAddr = "1.1.1.1".parse().expect("parse ip failed");
    println!("ip = {}", ip);

    // match处理Result类型
    let ip_addr: IpAddr = match "2.2.2.2".parse() {
        Ok(ip) => ip,
        Err(e) => panic!("parse ip failed: {:?}", e),
    };
    println!("ip_addr = {}", ip_addr);

    // if let处理Result类型
    let ip_addr: IpAddr = if let Ok(ip) = "3.3.3.3".parse() {
        ip
    } else {
        panic!("parse ip failed");
    };
    println!("ip_addr = {}", ip_addr);

    // 多层match嵌套处理多种类型的error
    let f = std::fs::File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => match std::fs::File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("create file failed: {:?}", e),
            },
            other_error => panic!("open file failed: {:?}", other_error),
        },
    };
    println!("f = {:?}", f);
}

use std::io::Read;

// 透传error
fn error_propagation() -> Result<String, std::io::Error> {
    let mut f = std::fs::File::open("hello.txt")?;
    println!("f = {:?}", f);
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    println!("s = {}", s);
    Ok(s)
}

// 链式调用透传error
fn error_propagation2() -> Result<String, std::io::Error> {
    let mut s = String::new();
    std::fs::File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 自定义错误类型
use std::fmt;

#[derive(Debug)]
struct CustomError {
    code: usize,
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.code {
            200 => write!(f, "success"),
            _ => write!(f, "error: code={}, message={}", self.code, self.message),
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError {
            code: 999,
            message: format!("io error[{}]", err).to_string(),
        }
    }
}

fn custom_error() {
    // 手动构造CustomError
    let err = CustomError {
        code: 200,
        message: "ok".to_string(),
    };
    println!("customize err = {}", err);
    println!("customize err display = {:?}", err);

    let err2 = CustomError {
        code: 500,
        message: "server error".to_string(),
    };
    println!("customize err2 = {}", err2);
    println!("customize err2 display = {:?}", err2);
}

fn from_error() -> Result<(), CustomError> {
    // 通过From trait将std::io::Error转换为CustomError
    // 使用?可以自动调用From trait
    let _file = std::fs::File::open("notexistfile.txt")?;

    Ok(())
}

// 统一化错误类型
// 1. 使用特征对象实现, Box<dyn Error>
fn diff_error_occurs() -> Result<String, Box<dyn std::error::Error>> {

    let file = std::env::var("notexistfile.txt")?;
    let source = std::fs::read_to_string(file)?;
    Ok(source)
}

// 2. 使用枚举实现, 配合自定义错误类型实现统一化
#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Env(std::env::VarError),
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<std::env::VarError> for MyError {
    fn from(err: std::env::VarError) -> Self {
        MyError::Env(err)
    }
}

// impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::Io(err) => write!(f, "io error: {}", err),
            MyError::Env(err) => write!(f, "env error: {}", err),
        }
    }
}

fn diff_error_occurs2() -> Result<String, MyError> {
    let file = std::env::var("TEST_FILE")?;
    let source = std::fs::read_to_string(file)?;
    Ok(source)
}


pub fn error() -> Result<(), MyError> {
    trigger_panic();
    handle_panic();

    let result = match error_propagation() {
        Ok(s) => s,
        Err(e) => panic!("error: {:?}", e),
    };
    println!("result = {:?}", result);

    let result2 = match error_propagation2() {
        Ok(s) => s,
        Err(e) => panic!("error: {:?}", e),
    };
    println!("result2 = {:?}", result2);

    custom_error();
    let e = from_error();
    println!("from_error = {:?}", e);

    let _ = diff_error_occurs();
    let content = diff_error_occurs2()?;
    println!("content = {}", content);

    Ok(())
}