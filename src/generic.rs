use crate::elevator::{Elevator, ElevatorWeightController};


impl Elevator {
    pub fn weight_add(&mut self, weight: u32) -> u32 {
        self.current_weight = self.current_weight + weight;
        self.current_weight
    }
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

    let elevator_weight = ElevatorWeightController {
        current_weight: 77.77,
        weight_limit: 1000.1,
    };
    println!("elevator_weight = {:?}", elevator_weight);

    // error: mismatched types, 两个泛型参数类型不一致
    // let elevator_weight = ElevatorWeightController {
    //     current_weight: 70,
    //     weight_limit: 1000.1,
    // };
    
    
}