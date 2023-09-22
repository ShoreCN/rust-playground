#[derive(Debug)]

struct Elevator {
    floor: u8,
    capacity: u8,
    occupants: u8,
}

// 通过构建函数创建结构体
fn build_elevator(floor: u8, capacity: u8) -> Elevator {
    Elevator {
        floor,
        capacity,
        occupants: 0,
    }
}

fn normal_struct() {
    let mut ele = Elevator {
        floor: 1,
        capacity: 10,
        occupants: 0,
    };

    println!("init elevator floor = {}, capacity = {}, occupants = {}",
             ele.floor, ele.capacity, ele.occupants);
    
    ele.floor = 10;
    ele.occupants = 5;
    println!("changed elevator floor = {}, capacity = {}, occupants = {}",
             ele.floor, ele.capacity, ele.occupants);

    let mut ele2 = build_elevator(ele.floor, ele.capacity);
    println!("ele2 floor = {}, capacity = {}, occupants = {}",
             ele2.floor, ele2.capacity, ele2.occupants);

    // 通过结构体更新语法创建新的结构体
    ele2.occupants = 3;
    let ele3 = Elevator {
        floor: 2,
        ..ele2
    };
    println!("ele3 floor = {}, capacity = {}, occupants = {}",
             ele3.floor, ele3.capacity, ele3.occupants);
}

fn tuple_struct() {
    struct Color(u8, u8, u8);
    struct Point(i32, i32, i32);

    let black = Color(1, 2, 3);
    let origin = Point(3, 2, 1);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {}, {})", origin.0, origin.1, origin.2);
}

fn unit_struct() {
    struct UnitStruct;
    let _unit = UnitStruct;
    // println!("unit struct = {:?}", unit);

}

fn strcut_display() {
    let ele = Elevator {
        floor: 1,
        capacity: 10,
        occupants: 0,
    };
    println!("display ele struct = {:?}", ele);
    println!("display ele struct friendly = {:#?}", ele);
    dbg!(ele);
}

pub fn struct_test() {
    normal_struct();

    tuple_struct();

    unit_struct();

    strcut_display();
}