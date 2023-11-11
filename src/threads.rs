use std::thread;
use crate::elevator::Elevator;
use rand;

impl Elevator {
    fn goto_random_floor(&mut self, elevator_num: u8) {
        println!("elevator[{elevator_num}] current floor is {}", self.current_floor);
        let random_floor = rand::random::<u8>() % 33 + 1;
        self.current_floor = random_floor;
        println!("elevator[{elevator_num}] go to floor {}", self.current_floor);
    }
}

fn elevators_process(){
    // 开启三个电梯线程来处理不同电梯的状态
    let mut threads = Vec::new();

    for i in 0..3 {
        let handler = thread::spawn(move || {
            let mut elevator = Elevator::new();
            elevator.goto_random_floor(i);
        });
        threads.push(handler);
    }

    // 等待所有线程结束
    for handler in threads {
        handler.join().unwrap();
    }
}

pub fn threads() {
    elevators_process();
}