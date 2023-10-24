use std::collections::HashMap;

use crate::elevator::FloorType;


fn array_iter(){
    let arr = [1, 2, 3, 4, 5];
    for _i in arr {
    }
    for _i in &arr {
    }
    for _i in arr.iter() {
    }
    for _i in arr.into_iter() {
    }

    let mut v = vec![1, 2, 3, 4, 5];
    // 使用for循环遍历vector之后, vector的所有权会被转移, 后续无法再次使用
    // for _i in v {
    // }
    for _i in &v {
    }
    for _i in v.iter() {
    }
    // 可变借用, 可以修改vector的值
    for _i in v.iter_mut() {
    }

    // 使用into_iter()方法, vector的所有权会被转移, 后续无法再次使用
    for _i in v.into_iter() {
    }

}

impl ToString for FloorType {
    fn to_string(&self) -> String {
        match self {
            FloorType::Bottom => String::from("Bottom"),
            FloorType::Normal => String::from("Normal"),
            FloorType::Top => String::from("Top"),
        }
    }
}

fn iter_usage() {
    let mut floors = vec![FloorType::Bottom, FloorType::Normal, FloorType::Top];
    // map是一个迭代器适配器, 会创建一个新的迭代器
    // map方法不会消耗原vector, 所以可以继续使用
    // map方法是惰性的, 只有在使用时才会执行, 所以在map方法使用之后再调用collect方法(消费者适配器)才算是进行了执行
    // collect方法可以将迭代器转换成所需要的集合类型, 在这里是转换成vector, 通过类型推导, 由编译器进行类型推导
    let floor_names: Vec<_> = floors.iter().map(|floor| floor.to_string()).collect();
    println!("floor_names = {:?}", floor_names);

    let prices = vec![100, 200, 300];
    let floor_price_hashmap: HashMap<_, _> = floor_names.iter().zip(prices.iter()).collect();
    println!("floor_price_hashmap = {:?}", floor_price_hashmap);
}

pub fn iterator(){
    array_iter();
    iter_usage();
}