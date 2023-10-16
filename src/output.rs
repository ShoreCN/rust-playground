

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
}

pub fn output() {
    standard_print();
}