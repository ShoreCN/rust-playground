
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
    created_at: i64,
}

// 为具有生命周期的结构体实现方法
impl<'a, T> ElevatorMaker<'a, T> {
    fn new(name: &T) -> ElevatorMaker<T> {
        ElevatorMaker { name, created_at: 0}
    }
}


pub fn lifetime() {
    let item1 = 1;
    let item2 = 2;
    let result = max(&item1, &item2);
    println!("{item1} and {item2} max is {result}");
    
    let maker;
    {
        let name = "Toshiba";
        maker = ElevatorMaker::new(&name);
        println!("maker name = {}", maker.name);
    }
    // 结构体成员引用的变量已经超出了作用域, 虽然结构体本身还在作用域内, 但是结构体成员引用的变量已经超出了作用域, 所以继续使用结构体会报错
    // println!("maker created at {}", maker.created_at);
}