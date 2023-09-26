
use crate::elevator;

impl elevator::Elevator {
    // 关联函数, 因为没有传递self, 调用的时候要用::, 而不是.
    pub fn new() -> elevator::Elevator {
        elevator::Elevator {
            current_floor: 1,
            direction: elevator::Direction::Down,
            destination: 1,
            state: elevator::State::Idle,
            current_weight: 0,
            weight_limit: 1000,
        }
    }

    pub fn handle_event(&mut self, event: elevator::Event) {
        match event {
            elevator::Event::Call(floor, direction) => {
                println!("Call: floor = {}, direction = {:?}", floor, direction);
                self.destination = floor;
                self.direction = direction;
                self.state = elevator::State::Moving;
            },
            elevator::Event::FloorReached(floor) => {
                println!("FloorReached: floor = {}", floor);
                self.current_floor = floor;
                if self.current_floor == self.destination {
                    self.state = elevator::State::Stopped;
                }
            },
            elevator::Event::Stop => {
                println!("Stop");
                self.state = elevator::State::Stopped;
            },
        }
    }

    pub fn state(&self) -> &elevator::State {
        &self.state
    }
}

impl elevator::State {
    pub fn is_moving(&self) -> bool {
        match self {
            elevator::State::Moving => true,
            _ => false,
        }
    }
}

pub fn method_test() {
    let mut elevator = elevator::Elevator::new();
    elevator.handle_event(elevator::Event::Call(3, elevator::Direction::Up));
    elevator.handle_event(elevator::Event::FloorReached(3));
    elevator.handle_event(elevator::Event::Stop);
    elevator.handle_event(elevator::Event::Call(1, elevator::Direction::Down));
    elevator.handle_event(elevator::Event::FloorReached(1));
    elevator.handle_event(elevator::Event::Stop);
    println!("elevator state = {:?}", elevator.state());
    println!("elevator is moving = {}", elevator.state().is_moving());
}