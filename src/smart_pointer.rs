
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
}

pub fn smart_pointer() {
    smart_pointer_box();
    customize_smart_pointer();
    reference_counting();
}