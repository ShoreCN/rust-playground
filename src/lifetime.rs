
// 具有生命周期的函数
fn max<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b {
        a
    } else {
        b
    }
}


// // 结构体中无法直接使用引用类型, 需要声明生命周期, 否则会报错missing lifetime specifier
// struct ElevatorMaker<T> {
//     name: &T,
//     created_at: i64,
// }

// 具有生命周期的结构体, 说明引用参数的生命周期需要比结构体的生命周期长
struct ElevatorMaker<'a, T> {
    name: &'a T,
    _created_at: i64,
}

// 为具有生命周期的结构体实现方法
impl<'a, T> ElevatorMaker<'a, T> {
    fn new(name: &T) -> ElevatorMaker<T> {
        ElevatorMaker { name, _created_at: 0}
    }

    fn update_name<'b>(&'a mut self, new_name: &'b T) -> &'b T 
    where
        'b: 'a, // 'b的生命周期必须比'a长
    {
        // println!("new name = {}", new_name);
        self.name = new_name;
        new_name
    }
}

fn print_s(s: &str) {
    println!("{}", s);
}

// 静态生命周期, 通过此方式声明的引用, 生命周期为整个程序的生命周期
fn static_lifetime() {
    let os;
    {
        // let s: &'static str = "Hello, world!";
        let s: &str = "Hello, world!";
        os = s;
        print_s(s);
    }
    println!("after lifetime of s, str is still live: {os}");

    let name;
    {
        static NAME: &str = "Toshiba";
        name = &NAME;
        println!("static NAME = {}, ref name = {name}", NAME);
    }
    println!("after lifetime of NAME, str is still live: {name}");

    // 如下例子无法通过编译, 因为引用的变量超出了作用域
    // let r;
    // {
    //     let s = "REF STR";
    //     r = &s;
    //     println!("ref = {}", r);
    // }
    // println!("after lifetime of ref, str is still live: {r}");
}


pub fn lifetime() {
    let item1 = 1;
    let item2 = 2;
    let result = max(&item1, &item2);
    println!("{item1} and {item2} max is {result}");
    
    let mut maker;
    {
        let name = "Toshiba";
        maker = ElevatorMaker::new(&name);
        println!("maker name = {}", maker.name);
        let new_name = "Mitsubishi";
        println!("new maker name = {}", maker.update_name(&new_name));
    }
    // 结构体成员引用的变量已经超出了作用域, 虽然结构体本身还在作用域内, 但是结构体成员引用的变量已经超出了作用域, 所以继续使用结构体会报错
    // println!("maker created at {}", maker.created_at);

    static_lifetime();
}