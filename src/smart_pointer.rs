
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

use std::fmt::Display;

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
}

pub fn smart_pointer() {
    smart_pointer_box();
}