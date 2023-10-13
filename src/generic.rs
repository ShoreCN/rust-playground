
use crate::elevator::{Elevator, ElevatorWeightController};


impl Elevator {
    pub fn weight_add(&mut self, weight: u32) -> u32 {
        self.current_weight = self.current_weight + weight;
        self.current_weight
    }
}

impl<T: std::ops::AddAssign + Copy> ElevatorWeightController<T> {
    fn get_current_weight(&self) -> &T {
        &self.current_weight
    }

    fn increase_weight(&mut self, weight: T) -> T {
        self.current_weight += weight;
        self.current_weight
    }
}

impl ElevatorWeightController<String> {
    // 通过指定泛型参数类型, 限制了该方法只能用于String类型对应的ElevatorWeightController
    fn get_weight_annotation(&self) -> String {
        format!("The elevator weight limit is {} kg, current weight is {} kg.", self.weight_limit, self.current_weight)
    }
}

// 对数组进行泛型, 并通过trait bound限制泛型参数类型
fn display_passenger_weight<T: std::fmt::Display>(passenger_array: &[T]) {
    for passenger in passenger_array {
        println!("passenger weight = {}", passenger);
    }
}

// 对数组进行泛型, 并通过const泛型参数限制数组长度
fn display_passenger_info<T: std::fmt::Display, const N: usize>(passenger_array: [T; N]){
    println!("passenger_array length = {}", N);
    for passenger in passenger_array {
        println!("passenger weight = {}", passenger);
    }
}

fn array_generic() {
    let passenger_array = [50, 60, 70, 80];
    display_passenger_weight(&passenger_array);
    let passenger_array = ["50", "60", "70", "80"];
    display_passenger_weight(&passenger_array);
    display_passenger_info(passenger_array)
}

pub fn generic() {
    let mut elevator = Elevator::new();
    elevator.weight_add(50);
    println!("increase 50 weight, elevator_weight = {}", elevator.current_weight);

    elevator.weight_add(100);
    println!("increase 100 weight, elevator_weight = {}", elevator.current_weight);

    let elevator_weight = ElevatorWeightController {
        current_weight: 100,
        weight_limit: 1000,
    };
    println!("elevator_weight = {:?}", elevator_weight);

    let mut elevator_weight = ElevatorWeightController {
        current_weight: 77.77,
        weight_limit: 1000.1,
    };
    println!("elevator_weight = {:?}", elevator_weight);

    // error: mismatched types, 两个泛型参数类型不一致
    // let elevator_weight = ElevatorWeightController {
    //     current_weight: 70,
    //     weight_limit: 1000.1,
    // };
    
    println!("get_current_weight= {:?}", elevator_weight.get_current_weight());
    elevator_weight.increase_weight(55.3);
    println!("get_current_weight= {:?}", elevator_weight.get_current_weight());

    let elevator_weight = ElevatorWeightController {
        current_weight: "77.77".to_string(),
        weight_limit: "1000.1".to_string(),
    };
    println!("get_weight_annotation = {}", elevator_weight.get_weight_annotation());

    array_generic();
}