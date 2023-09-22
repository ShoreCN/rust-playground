#[derive(Debug)]

enum 方向 {
    向上,
    向下,
    向左,
    向右,
}

enum DirectionWithData {
    Up(u8),
    _Down(u8),
    _Left(u8),
    _Right(u8),
}

pub fn enum_test() {
    println!("[enum_test]");

    let 方向 = 方向::向上;
    println!("方向 = {:?}", 方向);
    match 方向 {
        方向::向上 => println!("向上"),
        方向::向下 => println!("向下"),
        方向::向左 => println!("向左"),
        方向::向右 => println!("向右"),
    }
    println!("方向枚举包含4种情况: 1.{:?}, 2.{:?}, 3.{:?}, 4.{:?}", 方向::向上, 方向::向下, 方向::向左, 方向::向右);

    let direction = DirectionWithData::Up(2);
    // println!("direction = {:?}", direction);
    if let DirectionWithData::Up(n) = direction {
        println!("向上{}层", n);
    }

}