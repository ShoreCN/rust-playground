
use crate::elevator::{Elevator, ElevatorWeightController};

trait Annotation {
    // 声明一个方法, 该方法返回String类型, 不用实现, 所以使用;结尾
    fn get_annotation(&self) -> String;

    // 声明一个方法, 提供实现作为默认实现
    fn alarm(&self) {
        println!("Alarm!");
    }
}

trait Beep {
    // 声明一个方法
    fn beep(&self) {
        println!("Beep!");
    }
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

    // 重载 Annotation trait 的 alarm 方法
    fn alarm(&self) {
        println!("Alarm! The elevator weight is over the limit!");
    }
}

impl Beep for Elevator {
    // 实现 Beep trait, 该方法返回String类型
    fn beep(&self) {
        println!("Beep! Beep!");
    }
}

// 使用特征作为参数
fn print_annotation<T: Annotation>(item: &T) {
    println!("{}", item.get_annotation());
}

// 使用特征作为参数, 使用语法糖表达
fn print_annotation_with_syntax_sugar(item: &impl Annotation) {
    println!("{}", item.get_annotation());
}

// 多重约束
fn beep_and_alarm<T: Annotation + Beep>(item: &T) {
    item.beep();
    item.alarm();
}

// 多重约束使用语法糖表达
fn beep_and_alarm_with_syntax_sugar(item: &(impl Annotation + Beep)) {
    item.beep();
    item.alarm();
}

// 使用where子句来简化多重约束
fn print_annotation_and_beep<T, U>(item1: &T, item2: &U)
where
    T: Annotation,
    U: Beep,
{
    println!("{}", item1.get_annotation());
    item1.alarm();
    item2.beep();
}

// 通过特征约束做到对满足特定条件的类型实现方法
struct ElevatorRemark<T> {
    remark: T,
    timestamp: u64,
}

impl<T> ElevatorRemark<T> {
    fn new(remark: T) -> Self {
        Self {
            remark,
            timestamp: 0,
        }
    }
}

impl<T: std::fmt::Display> ElevatorRemark<T> {
    fn print_remark(&self) {
        println!("remark: {}, at {}", self.remark, self.timestamp)
    }
}


// 通过返回impl trait来简化代码
fn create_elevator() -> impl Annotation {
    Elevator::new()
}

// 使用derive来自动实现特征
#[derive(Debug, Clone, Copy)]
struct ElevatorRemark2<T> {
    _remark: T,
    _timestamp: u64,
}

pub fn trait_test() {
    let elevator = Elevator::new();
    println!("{}", elevator.get_annotation());
    elevator.alarm();

    let elevator_weight_controller = ElevatorWeightController {
        current_weight: 0,
        weight_limit: 1000,
    };
    println!("{}", elevator_weight_controller.get_annotation());
    elevator_weight_controller.alarm();

    print_annotation(&elevator);
    print_annotation_with_syntax_sugar(&elevator);
    beep_and_alarm(&elevator);
    beep_and_alarm_with_syntax_sugar(&elevator);

    print_annotation_and_beep(&elevator_weight_controller, &elevator);

    let remark = ElevatorRemark::new("The elevator is overloaded!");
    remark.print_remark();
    let remark = ElevatorRemark::new(123);
    remark.print_remark();
    // init remark by a param without Display trait
    let _remark = ElevatorRemark::new(elevator);
    // _remark.print_remark();
    // the method `print_remark` exists for struct `ElevatorRemark<Elevator>`, but its trait bounds were not satisfied
    // method cannot be called on `ElevatorRemark<Elevator>` due to unsatisfied trait bounds

    let _elevator = create_elevator();
}