use std::thread;
use crate::elevator::{Elevator, State};
use rand;
use std::sync::{Arc, Barrier};
use std::cell::RefCell;

// 线程局部变量
thread_local!(static INIT_WEIGHT: RefCell<u32> = RefCell::new(1000));

impl Elevator {
    fn goto_random_floor(&mut self, elevator_num: u8) {
        println!("elevator[{elevator_num}] current floor is {}", self.current_floor);
        let random_floor = rand::random::<u8>() % 33 + 1;
        self.current_floor = random_floor;
        self.state = State::Moving;
        INIT_WEIGHT.with(|weight| {
            self.current_weight = *weight.borrow();
        });
        println!("elevator[{elevator_num}] go to floor {}", self.current_floor);
    }
}

fn elevators_process(){
    // 开启三个电梯线程来处理不同电梯的状态
    let mut threads = Vec::new();
    let threads_num = 3;
    let barrier = Arc::new(Barrier::new(threads_num));

    INIT_WEIGHT.with(|weight| {
        // 变更线程局部变量的值
        *weight.borrow_mut() = 2000;
        println!("modified weight = {}", *weight.borrow());
    });

    for i in 0..threads_num as u8 {
        let thread_barrier = barrier.clone();
        let handler = thread::spawn(move || {
            let mut elevator = Elevator::new();
            elevator.goto_random_floor(i);
            // 为了让所有的电梯状态打印都在最后, 使用线程屏障控制线程执行到此处进行等待
            thread_barrier.wait();
            println!("elevator[{i}] state is {:?}, weight is {}", elevator.state(), elevator.current_weight);
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