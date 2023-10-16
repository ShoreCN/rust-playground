
#[derive(Debug)]
struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

struct CustomiseTime {
    hour: u8,
    minute: u8,
    second: u8,
}

impl std::fmt::Display for CustomiseTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

fn standard_print() {
    println!("standard {}", "print:");
    println!("the symbol to debug: {:?}", ("debug", "print"));
    println!("interger output: {:04}", 100);
    println!("float output: {:.2}", 3.1415926);

    // error output
    eprintln!("error output: {}", "when error occurs");

    // position parameter
    println!(
        "{0} is {1} and {0} is {2}",
        "this", "a string", "another string"
    );

    // named parameter
    println!(
        "{subject} {verb} {object}",
        subject = "the quick brown fox",
        verb = "jumps over",
        object = "the lazy dog"
    );
    
    let pi = 3.1415926;
    println!("pi is roughly {pi:.3}", pi = pi);
}

// 为外部类型实现Display特征, 通过impl newtype pattern来实现
// failed to implement Display trait for Vec<String>, because it's not our own type
// impl fmt::Display for Vec<String> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.join(", "))
//     }
// }

use std::fmt;
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn customise_print() {
    // 当一个类型实现了Display特征, 可以使用{}来输出, 实现了Debug特征, 可以使用{:?}来输出
    let s = Time {
        hour: 1,
        minute: 23,
        second: 56,
    };
    println!("struct output: {:?}", s);

    // human readable output
    println!("human readable output: {:#?}", s);

    let cs = CustomiseTime {
        hour: 1,
        minute: 23,
        second: 56,
    };
    println!("customise struct output: {}", cs);
    println!("human readable customise struct output: {:#}", cs);

    let w = Wrapper(vec![String::from("Elevator"), String::from("coming")]);
    println!("customise struct output: {}", w);
}

pub fn output() {
    standard_print();
    customise_print();
}