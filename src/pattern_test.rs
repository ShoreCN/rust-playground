
use crate::elevator;


fn while_let () {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 循环直至 stack 为空结束
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
}

fn if_let () {
    let option = Some(7);

    // match/for/while/let都需要完全匹配才可通过编译(不可驳模式匹配)
    match option {
        Some(i) => println!("match some {}", i),
        None => println!("match none"),
    }

    // if let可以不完全匹配(可驳模式匹配)
    if let Some(i) = option {
        println!("if let some {}", i);
    }
}

fn match_multiple_pattern () {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3..=5 => println!("three through five"),
        _ => println!("others"),
    }

    let c = 'c';
    match c {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("others"),
    }
}

fn destruct_struct () {
    struct ElevatorButton {
        floor: u8,
        panel_id: char,
    }

    let button = ElevatorButton { floor: 3, panel_id: 'c' };
    let ElevatorButton { floor: a, panel_id: b } = button;
    println!("floor = {}, panel_id = {}", a, b);
    let ElevatorButton { floor, panel_id } = button;
    println!("floor = {}, panel_id = {}", floor, panel_id);

    match button {
        ElevatorButton { floor: 1|2, panel_id } => println!("first or second floor, panel_id = {}", panel_id),
        ElevatorButton { floor, panel_id: 'c' } => println!("third floor, No. {}", floor),
        ElevatorButton { floor, panel_id } => println!("others, floor = {}, panel_id = {}", floor, panel_id),
    }
}

fn desctruct_enum () {
    let elevator_event = elevator::Event::Call(3, elevator::Direction::Up);

    match elevator_event {
        elevator::Event::Call(floor, direction) => println!("Call: floor = {}, direction = {:?}", floor, direction),
        elevator::Event::FloorReached(floor) => println!("FloorReached: floor = {}", floor),
        elevator::Event::Stop => println!("Stop"),
    }

}

fn destruct_complex_data () {

    // 解构结构体和枚举嵌套的情况
    let elevator = elevator::Elevator {
        current_floor: 1,
        direction: elevator::Direction::Up,
        destination: 3,
        state: elevator::State::Overload(5),
    };
    if let elevator::Elevator { current_floor, direction, destination, state: elevator::State::Overload(overload) } = elevator {
        print!("destrcut elevator: current_floor = {}, direction = {:?}, destination = {}, overload = {}", current_floor, direction, destination, overload);
    }
}

pub fn pattern_test() {
    if_let();
    while_let();
    match_multiple_pattern();
    destruct_struct();
    desctruct_enum();
    destruct_complex_data();
}