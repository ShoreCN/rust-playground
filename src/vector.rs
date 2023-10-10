
fn vector_creation() {
    // 预先声明类型
    let v: Vec<String> = Vec::new();
    println!("v = {:?}", v);

    // 如果编译器可以推导出类型, 可以省略类型声明
    let mut v = Vec::new();
    v.push(100);
    println!("v = {:?}", v);

    // 提前预估动态数组大小, 可以避免频繁的重新分配内存
    let mut v = Vec::with_capacity(10);
    v.push("100");
    println!("v = {:?}", v);

    // 使用vec宏来创建动态数组
    let v = vec![1, 2, 3];
    let first = &v[0];
    let second = &v[1];
    let last = &v[v.len() - 1];
    println!("first = {first}, second = {second}, last = {last}");

    // 使用get方法来获取数组元素, 返回Option<&T>, 需要解构处理获取其中的值
    let first = v.get(0);
    println!("first = {:?}", first);
    if let Some(i) = v.get(3) {
        println!("third item is {i}");
    } else {
        println!("no third item");
    }

    // 使用下标获取动态数据元素比较简洁, 但是会导致数组越界panic, 适合已经明确操作安全的场景
    // 使用.get()方法获取动态数组元素, 会返回Option<&T>, 需要解构处理获取其中的值, 适合不确定操作安全的场景
}


pub fn vector() {
    vector_creation();
}