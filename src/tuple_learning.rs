fn test_tuple_return(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

pub fn tuple_learning() {
    println!("[tuple_learning]");

    let tup = (1, 2, 3);
    println!("tup = {:?}", tup);
    // 解构
    let (x, y, z) = tup;
    println!("tuple after destructuring, x = {}, y = {}, z = {}", x, y, z);

    // 访问元组元素, 通过索引, 索引从0开始
    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);

    // 返回值是元组
    let (s, len) = test_tuple_return(String::from("My first rust program"));
    println!("s = {}, len = {}", s, len);
}