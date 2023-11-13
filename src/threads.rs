use std::thread;
use crate::elevator::{Elevator, State};
use rand;
use std::sync::{Arc, Barrier, mpsc};
use std::cell::RefCell;

// 线程局部变量
thread_local!(static INIT_WEIGHT: RefCell<u32> = RefCell::new(1000));

impl Elevator {
    fn goto_random_floor(&mut self, elevator_num: u8) -> Result<String, &'static str> {
        println!("elevator[{elevator_num}] current floor is {}", self.current_floor);
        let random_floor = rand::random::<u8>() % 33 + 1;
        self.current_floor = random_floor;
        self.state = State::Moving;
        INIT_WEIGHT.with(|weight| {
            self.current_weight = *weight.borrow() + elevator_num as u32 * 100;
        });
        println!("elevator[{elevator_num}] go to floor {}", random_floor);
        Ok(format!("Nofitication: Elevator[{}] arrived {}.", elevator_num, random_floor))
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

    // 创建消息通道
    let (tx, rx) = mpsc::channel();
    tx.send("Notification init".to_string()).unwrap();

    for i in 0..threads_num as u8 {
        let thread_barrier = barrier.clone();
        let multi_tx = tx.clone();
        let mut elevator = Elevator::new();
        let handler = thread::spawn(move || {
            let r = elevator.goto_random_floor(i);
            // 如果goto_random_floor()返回的是Ok, 则说明电梯已经到达指定楼层, 通过消息通道通知主线程
            if let Ok(msg) = r {
                multi_tx.send(msg).unwrap();
            }
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

    // 所有的tx都drop掉之后, 使用for循环接收消息通道中的消息才会结束
    // tx的clone在线程中会被drop掉, 所以这里只需要drop掉主线程中的tx即可
    drop(tx);

    // 接收消息通道中的消息
    for received in rx {
        println!("Message received >> {}", received);
    }

    // 子线程使用完线程局部变量之后再查看一下线程局部变量的值
    // 可以看出线程局部变量的值并没有被修改, 在不同的线程中, 线程局部变量的值是独立的
    INIT_WEIGHT.with(|weight| {
        println!("final weight = {}", *weight.borrow());
    });
}

pub fn threads() {
    elevators_process();
}