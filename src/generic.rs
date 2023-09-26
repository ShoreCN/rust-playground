use crate::elevator::Elevator;


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
}