
// 因为结构中使用了自身, 所以编译时无法确定结构的大小, 属于动态大小类型, 从而导致编译错误
// enum WrongList {
//     Cons(i32, WrongList),
//     Nil,
// }

// 使用Box可以将动态大小类型转换为固定大小类型
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}


trait DisplayContent {
    fn display(&self);
}

use crate::elevator::{Elevator, ElevatorWeightController};
impl DisplayContent for Elevator {
    fn display(&self) {
        println!("The elevator now is {:?}, current floor is {}, current direction is {:?}, current destination is {}, current weight is {} kg, weight limit is {} kg.", self.state, self.current_floor, self.direction, self.destination, self.current_weight, self.weight_limit);
    }
}

impl DisplayContent for ElevatorWeightController<u32> {
    fn display(&self) {
        println!("The elevator weight limit is {} kg, current weight is {} kg.", self.weight_limit, self.current_weight);
    }
}

#[derive(Debug)]
struct GlobalConfig {
    pub name: String,
    pub value: String,
}

fn get_global_usage_var() -> &'static GlobalConfig {
    let mut var = GlobalConfig {
        name: String::from("global_usage_var_name"),
        value: String::from("global_usage_var_value"),
    };
    Box::leak(Box::new(var))
}

fn smart_pointer_box() {
    // 通过Box可以将DST(即动态类型)转换为固定大小的类型
    // DST: 动态大小类型, 即运行时才能确定大小的类型
    // DST: [T], str, trait
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("customize list = {:?}", list);

    // 通过Box还可以将特征对象转换为固定大小的类型, 实现一个存放不同结构对象的数组
    let elevator1 = Elevator::new();
    let elevator2 = Elevator::new();
    let controller = ElevatorWeightController {
        current_weight: 0,
        weight_limit: 1000,
    };
    let display_obj_arr: Vec<Box<dyn DisplayContent>> = vec![
        Box::new(elevator1),
        Box::new(elevator2),
        Box::new(controller),
    ];
    for obj in display_obj_arr {
        obj.display();
    }

    let global_config = get_global_usage_var();
    println!("global_config_name = {}, global_config_value = {}", global_config.name, global_config.value);
}


use std::{ops::Deref, rc::Rc};

struct FloorButton(u32);

impl FloorButton {
    fn new(floor: u32) -> FloorButton {
        FloorButton(floor)
    }
}

impl Deref for FloorButton {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for FloorButton {
    fn drop(&mut self) {
        println!("FloorButton {} is dropped.", self.0);
    }
}

fn customize_smart_pointer() {
    let floor_button = FloorButton::new(10);
    println!("floor_button = {}", *floor_button);

    // error: 无法直接调用对象的drop方法
    // floor_button.drop();

    // 通过std::mem::drop可以调用对象的drop方法
    // 本质上是通过将参数的所有权转移给drop函数, 从而使得floor_button对象无法再被使用, 离开作用域后会被自动drop
    drop(floor_button);

    // panic: borrow of moved value: `floor_button`
    // println!("use floor_button after drop. floor_button = {}", *floor_button);
}

struct ElevatorPassenger {
    elevator_number: Rc<u8>
}

fn reference_counting() {
    let e1 = Rc::new(Elevator::new());
    println!("e1 counting = {}", Rc::strong_count(&e1));
    
    let e2 = Rc::clone(&e1);
    println!("e2 counting = {}", Rc::strong_count(&e2));
    println!("e1 counting = {}", Rc::strong_count(&e1));

    {
        let e3 = Rc::clone(&e1);
        println!("e3 counting = {}", Rc::strong_count(&e3));
    }
    // 变量离开作用域后, 引用计数会减1
    println!("e1 counting = {}", Rc::strong_count(&e1));

    // Rc是指向变量的不可变引用, 所以无法修改变量的值
    // e1.current_floor = 2;

    let elevator_num = 1;
    let p1 = ElevatorPassenger {
        elevator_number: Rc::new(elevator_num),
    };
    let p2 = ElevatorPassenger {
        elevator_number: Rc::clone(&p1.elevator_number),
    };
    println!(
        "p1 elevator_number = {}, p2 elevator_number = {}, num{} elevator passenger count = {}", 
        p1.elevator_number, 
        p2.elevator_number, 
        elevator_num, 
        Rc::strong_count(&p1.elevator_number)
    );

    // 当所有对Rc的引用离开作用域后, 引用计数会减1, 当引用计数为0时, Rc会自动调用drop方法, 从而释放内存

    // Rc<T>只能用于单线程, 如果需要在多线程中共享数据, 可以使用Arc<T>
}

use std::sync::Arc;
use std::thread;

fn arc() {
    // 跨线程操作时, 需要使用Arc<T>

    let s = Arc::new(String::from("cross thread string"));
    for _ in 0..3 {
        let thread_s = Arc::clone(&s);
        thread::spawn(move || {
            println!("content in the thread {:?} is \"{}\"", thread::current().id(), thread_s);
        });
        // 注意线程回调函数中的参数是move的, 所以这里无法再使用thread_s
        // println!("content in the thread {:?} is \"{}\"", thread::current().id(), thread_s);
    }
    println!("counting = {}", Arc::strong_count(&s));
    // sleep, 主线程结束之后, 子线程可能还没有执行完, 从而导致子线程无法打印内容
    thread::sleep(std::time::Duration::from_millis(200));

    // 更好的选择是在主线程使用join方法, 等待子线程执行完毕
    let s = Arc::new(String::from("cross thread string 2"));
    let time_cost = std::time::Instant::now();
    let mut threads = vec![];
    for _ in 0..3 {
        let thread_s = Arc::clone(&s);
        let thread = thread::spawn(move || {
            println!("content in the thread {:?} is \"{}\"", thread::current().id(), thread_s);
            thread::sleep(std::time::Duration::from_millis(100));
        });
        threads.push(thread);
        // thread.join().unwrap(); // 在这里直接使用join, 会导致线程串行执行, 无法体现多线程的优势
        println!("time cost = {:?}", time_cost.elapsed());
    }
    for thread in threads {
        thread.join().unwrap();
    }
    println!("counting = {}", Arc::strong_count(&s));
    println!("time cost = {:?}", time_cost.elapsed());
}

use std::cell::{Cell, RefCell};


fn modify_immutable_variable() {
    // Rust中的Cell和RefCell可以在不可变变量中修改值
    let m = Cell::new(1);
    println!("old m = {}", m.get());
    m.set(2);
    println!("new m = {}", m.get());

    // 因为Cell只能存储Copy类型的值, 所以下面的代码无法通过编译
    // let m2 = Cell::new(String::from("hello"));
    // println!("old m2 = {}", m2.get());
    // m2.set(String::from("world"));
    // println!("new m2 = {}", m2.get());

    // 相比Cell, 更常用的是RefCell, 因为Cell只能存储Copy类型的值, 而RefCell可以存储任意类型的值
    let rc = RefCell::new(String::from("hello"));
    println!("old rc = {}", rc.borrow());
    *rc.borrow_mut() = String::from("world");
    println!("new rc = {}", rc.borrow());

    // 然而RefCell只是将可变借用的检查推迟到了运行时
    // 所以如果在运行时发生了可变借用的冲突(例如下面例子中的多次可变借用), 还是会导致程序崩溃panic
    // let rc2 = RefCell::new(String::from("hello"));
    // let r1 = rc2.borrow_mut();
    // let r2 = rc2.borrow_mut(); // panic: already borrowed: BorrowMutError
    // println!("r1 = {}, r2 = {}", r1, r2);
}


pub fn smart_pointer() {
    smart_pointer_box();
    customize_smart_pointer();
    reference_counting();
    arc();
    modify_immutable_variable();
}