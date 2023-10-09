
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

    fn tictak() {
        println!("TicTak from trait!");
    }
}

impl Annotation for Elevator {
    // 实现 Annotation trait, 该方法返回String类型
    fn get_annotation(&self) -> String {
        format!("[trait method] The elevator now is {:?}, current floor is {}, current direction is {:?}, current destination is {}, current weight is {} kg, weight limit is {} kg.", self.state, self.current_floor, self.direction, self.destination, self.current_weight, self.weight_limit)
    }
}

// 类型中存在同名方法
impl Elevator {
    fn get_annotation(&self) -> String {
        format!("[struct method] The elevator now is {:?}, current floor is {}, current direction is {:?}, current destination is {}, current weight is {} kg, weight limit is {} kg.", self.state, self.current_floor, self.direction, self.destination, self.current_weight, self.weight_limit)
    }

    fn tictak() {
        println!("TicTak from struct!");
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

// 特征对象
trait Brief {
    fn brief(&self) -> String;
}

// 通过特征对象来实现多态, 使用泛型的话只能保证同一类型
// 通过dyn关键字来声明特征对象, 即动态分发(dynamic dispatch)
struct BuildingStructure {
    floors: Vec<Box<dyn Brief>>,
    // floors: Vec<&dyn Brief>,
}

struct OfficeFloor {
    floor_number: u8,
    company_name: String,
}

struct ResidentialFloor {
    floor_number: u8,
    room_number: u8,
}

impl Brief for OfficeFloor {
    fn brief(&self) -> String {
        format!("[OfficeFloor] floor_number = {}, company_name = {}", self.floor_number, self.company_name)
    }
}

impl Brief for ResidentialFloor {
    fn brief(&self) -> String {
        format!("[ResidentialFloor] floor_number = {}, room_number = {}", self.floor_number, self.room_number)
    }
}

fn get_brief(x: Box<dyn Brief>) -> String {
    format!("Floor brief: {}", x.brief())
}

// fn get_brief(x: &dyn Brief) -> String {
//     x.brief()
// }

// 关联类型, 可以增加代码的可读性, 减少代码的重复, 相比泛型, 关联类型更加灵活
trait ShortBrief {
    type Output;
    fn short_brief(&self) -> Self::Output;
}

// 泛型参数的默认类型
trait FloorCombination<T = Self> {
    type Output;
    fn add(&self, other: &T) -> Self::Output;
}

impl FloorCombination<ResidentialFloor> for OfficeFloor {
    type Output = String;
    fn add(&self, other: &ResidentialFloor) -> Self::Output {
        format!("OfficeFloor {} + ResidentialFloor {}", self.floor_number, other.floor_number)
    }
}

// super trait, 用于扩展已有的trait
trait SuperBrief: Brief {
    fn super_brief(&self) -> String {
        format!("super brief: {}", self.brief())
    }
}

impl SuperBrief for OfficeFloor {}

struct _CookingFloor {
    floor_number: u8,
    kitchen_number: u8,
}

// 因为CookingFloor没有实现Brief trait, 所以无法实现SuperBrief trait
// panic: the trait bound `CookingFloor: Brief` is not satisfied
// impl SuperBrief for CookingFloor {}


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

    let elevator = create_elevator();

    let building = BuildingStructure {
        floors: vec![
            Box::new(OfficeFloor {
                floor_number: 1,
                company_name: String::from("ABC"),
            }),
            Box::new(ResidentialFloor {
                floor_number: 2,
                room_number: 3,
            }),
        ],
    };
    println!("building brief:");
    for floor in building.floors {
        println!("{}", get_brief(floor));
    }
    
    let first_floor = OfficeFloor {
        floor_number: 1,
        company_name: String::from("ABC"),
    };
    let second_floor = ResidentialFloor {
        floor_number: 2,
        room_number: 3,
    };
    println!("floor combination result: {}", first_floor.add(&second_floor));

    // elevator created by create_elevator() is a trait object
    println!("Annotation from trait method: {}", elevator.get_annotation());
    let elevator2 = Elevator::new();
    println!("Annotation from struct method: {}", elevator2.get_annotation());
    println!("Annotation from struct method: {}", Elevator::get_annotation(&elevator2));
    println!("Annotation from trait object: {}", Annotation::get_annotation(&elevator2));
    
    Elevator::tictak();
    // Beep::tictak();  // 无法直接调用trait里面的关联函数(没有指定self的函数)
    // 完全限定语法, 通过特征名字来调用关联函数
    <Elevator as Beep>::tictak();
}