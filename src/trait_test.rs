
use crate::elevator::{Elevator, ElevatorWeightController};

trait Annotation {
    // 声明一个方法, 该方法返回String类型, 不用实现, 所以使用;结尾
    fn get_annotation(&self) -> String;
}

impl Annotation for Elevator {
    // 实现 Annotation trait, 该方法返回String类型
    fn get_annotation(&self) -> String {
        format!("The elevator now is {:?}, current floor is {}, current direction is {:?}, current destination is {}, current weight is {} kg, weight limit is {} kg.", self.state, self.current_floor, self.direction, self.destination, self.current_weight, self.weight_limit)
    }
}

impl Annotation for ElevatorWeightController<u32> {
    // 实现 Annotation trait, 该方法返回String类型
    fn get_annotation(&self) -> String {
        format!("The elevator weight limit is {} kg, current weight is {} kg.", self.weight_limit, self.current_weight)
    }
}

pub fn trait_test() {
    let elevator = Elevator::new();
    println!("{}", elevator.get_annotation());

    let elevator_weight_controller = ElevatorWeightController {
        current_weight: 0,
        weight_limit: 1000,
    };
    println!("{}", elevator_weight_controller.get_annotation());
}