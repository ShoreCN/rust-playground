use std::collections::HashMap;

fn hashmap_creation() {
    let mut hm = HashMap::new();
    // hashmap的key和value都需要是相同类型
    hm.insert(String::from("key"), String::from("value"));
    hm.insert(String::from("key2"), String::from("value2"));
    println!("hm = {:?}", hm);

    let mut hm_with_capacity = HashMap::with_capacity(10);
    hm_with_capacity.insert(1, 1);
    hm_with_capacity.insert(2, 2);
    println!("hm_with_capacity = {:?}", hm_with_capacity);

    // 使用collect方法创建hashmap
    let keys = vec![String::from("key1"), String::from("key2")];
    let values = vec![String::from("value1"), String::from("value2")];
    let hm_from_collect: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    println!("hm_from_collect = {:?}", hm_from_collect);
}

fn hashmap_usage() {
    let hm = HashMap::from([("first", 1), ("second", 2)]);
    println!("hm = {:?}", hm);
    // 获取hashmap中的值
    let first = hm.get("first");
    println!("first = {:?}", first);
    // 获取hashmap中的值, 如果不存在, 则返回默认值
    let third = hm.get("third").unwrap_or(&0);
    println!("third = {:?}", third);

    let mut elevator_map = HashMap::from([("first", String::from("office floor")), ("second", String::from("residential floor"))]);
    println!("elevator_map = {:?}", elevator_map);
    let first = elevator_map.get("first");
    println!("first floor = {:?}", first);
    let unkown = String::from("unknown");
    let third = elevator_map.get("third").unwrap_or(&unkown);
    println!("third floor = {:?}", third);

    // 遍历hashmap
    for (key, value) in &elevator_map {
        println!("{}: {}", key, value);
    }

    // 修改hashmap中的值
    elevator_map.insert("third", String::from("Entertainment floor"));
    elevator_map.insert("third", String::from("Enterprise floor")); //modify
    elevator_map.entry("forth").or_insert(String::from("Security floor"));
    elevator_map.entry("forth").or_insert(String::from("Boss floor")); //no modify
    println!("elevator_map = {:?}", elevator_map);

}

pub fn hashmap(){
    hashmap_creation();
    hashmap_usage();
}