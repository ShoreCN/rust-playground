use std::thread;
use crate::elevator::{Elevator, State};
use rand;
use std::sync::{Arc, Barrier, mpsc, Mutex, RwLock, Condvar};
use std::cell::RefCell;
use std::time::Duration;

// 线程局部变量
thread_local!(static INIT_WEIGHT: RefCell<u32> = RefCell::new(1000));

impl Elevator {
    fn goto_random_floor(&mut self, elevator_num: u8) -> Result<(u8, String), &'static str> {
        println!("elevator[{elevator_num}] current floor is {}", self.current_floor);
        let random_floor = rand::random::<u8>() % 33 + 1;
        self.current_floor = random_floor;
        self.state = State::Moving;
        INIT_WEIGHT.with(|weight| {
            self.current_weight = *weight.borrow() + elevator_num as u32 * 100;
        });
        println!("elevator[{elevator_num}] go to floor {}", random_floor);
        Ok((random_floor, format!("Nofitication: Elevator[{}] arrived {}.", elevator_num, random_floor)))
    }
}

fn elevators_process(){
    // 开启三个电梯线程来处理不同电梯的状态
    let mut threads = Vec::new();
    let threads_num = 3;
    let barrier = Arc::new(Barrier::new(threads_num));
    let floor_counter = Arc::new(Mutex::new(0));

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
        let floor_counter = floor_counter.clone();
        let handler = thread::spawn(move || {
            let r = elevator.goto_random_floor(i);
            // 如果goto_random_floor()返回的是Ok, 则说明电梯已经到达指定楼层, 通过消息通道通知主线程
            if let Ok((dst_floor, msg)) = r {
                multi_tx.send(msg).unwrap();
                
                // 通过锁来控制多个线程对同一个变量的访问
                let mut counter = floor_counter.lock().unwrap();
                *counter += dst_floor;

                // 发送之后对象的所有权会被转移, 所以这里不能再次使用
                // 除非所发送对象是实现了Copy trait的类型, 例如int之类
                // println!("msg is {msg}");
            }
            // 为了让所有的电梯状态打印都在最后, 使用线程屏障控制线程执行到此处进行等待
            thread_barrier.wait();
            println!("elevator[{i}] state is {:?}, weight is {}", elevator.state(), elevator.current_weight);
        });
        threads.push(handler);
    }

    // 等待所有线程结束
    // 使用了线程屏障, 所以这里可以不用等待
    // for handler in threads {
    //     handler.join().unwrap();
    // }

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

    // 使用try_lock()方法来获取锁, 如果锁已经被其他线程获取, 则返回错误
    // 该方法不会阻塞当前线程
    println!("floor counter = {}", *floor_counter.try_lock().unwrap());
}

// 线程同步发送消息
fn thread_sync_msg() {
    println!("thread sync msg");
    // bound参数表示消息通道中最多可以存储(即缓存)的消息数量, 当消息数量超过bound时, send()方法才会开始阻塞
    // 如果bound=0, 那么send方法不会阻塞, 效果类似于异步消息通道
    // 通过设置bound参数可以控制消息通道内的消息数量, 防止消息通道内的消息过多导致内存占用过大
    let (tx, rx) = mpsc::sync_channel(0);
    
    let mut elevator = Elevator::new();
    let handler = thread::spawn(move || {
        let r = elevator.goto_random_floor(0);
        if let Ok(msg) = r {
            println!("Ready to send message. [triggered before sync send]");
            tx.send(msg).unwrap();
            println!("Elevator has sent message. [triggered after sync send]");
        }
    });

    println!("Ready to receive message. [triggered before sync receive]");
    thread::sleep(Duration::from_secs(1));

    let (_, received) = rx.recv().unwrap();
    println!("Message received >> {}", received);

    handler.join().unwrap();
}

// 通过枚举类型在消息发送中传递不同类型的数据
// 缺点: 因为枚举类型在内存中的数据对齐方式是按照最大成员的大小进行对齐的, 所以会造成内存空间的浪费
enum Msg {
    Text(String),
    Code(u32),
}

fn different_data_send() {
    let (tx, rx) = mpsc::channel();
    let mut elevator = Elevator::new();
    let handler = thread::spawn(move || {
        let r = elevator.goto_random_floor(0);
        if let Ok((_, msg)) = r {
            tx.send(Msg::Code(200)).unwrap();
            tx.send(Msg::Text(msg)).unwrap();
        }
    });

    // let received = rx.recv().unwrap();
    for received in rx {
        match received {
            Msg::Text(msg) => println!("Message received >> {}", msg),
            Msg::Code(i) => println!("Message received >> Code[{}]", i),
        }
    }

    handler.join().unwrap();
}

// 当多个线程都要进行数据读取时, 可以使用RwLock来进行读取, 这样可以提高效率
fn rwlock_test() {
    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        println!("r1 = {}", r1);
        let r2 = lock.read().unwrap();
        println!("r2 = {}", r2);
        // println!("r1 = {}, r2 = {}", r1, r2);

        // 已经获取了读锁的线程, 不能再获取写锁, 否则会造成死锁
        // let mut m1 = lock.try_write().unwrap();
        // *m1 += 1;
        // println!("m1 = {}", m1);
    }
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        println!("w = {}", w);

        // 已经获取了写锁的线程, 不能再获取读锁或者写锁, 否则都会造成死锁
        // thread 'main' panicked at 'rwlock read lock would result in deadlock'
        // let mut w2 = lock.write().unwrap();
        // *w2 += 1;
        // println!("w2 = {}", w2);

        // let r1 = lock.read();
        // println!("r1 = {:?}", r1);
    }
}

// 通过条件变量来控制线程之间的执行顺序
// 例如本例通过CondVar来实现执行顺序:
// 0. 子线程业务执行;
// 1. 主线程业务执行;
// 2. 子线程收尾;
// 3. 主线程收尾;
fn condvar_usage() {
    let counter = Arc::new(Mutex::new(0));
    let cond = Arc::new(Condvar::new());
    let ccounter = counter.clone();
    let ccond = cond.clone();

    // 该线程的业务流程开始和结束时都进行一次计数, 不过结束的计数需要等待通知
    let handler = thread::spawn(move || {
        let mut counter = ccounter.lock().unwrap();

        // process...
        println!("sub process...");
        *counter += 1;
        // 通知主线程
        ccond.notify_one();

        // 业务流程结束, 等待主线程通知后进行收尾流程
        while *counter < 2 {
            counter = ccond.wait(counter).unwrap();
        }
        println!("sub process end...");
        *counter += 1;
        ccond.notify_one();
    });

    // 等待业务流程开始
    let mut counter = counter.lock().unwrap();
    while *counter < 1 {
        counter = cond.wait(counter).unwrap();
    }
    println!("counter = {}", *counter);

    // 主线程的业务流程
    println!("main process...");
    *counter += 1;
    cond.notify_one();

    // 等待业务流程结束
    while *counter < 3 {
        counter = cond.wait(counter).unwrap();
    }
    println!("counter = {}", *counter);
    println!("main process end...");

}

pub fn threads() {
    elevators_process();
    thread_sync_msg();
    different_data_send();
    rwlock_test();
    condvar_usage();
}