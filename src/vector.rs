
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


fn vector_iteration() {
    let mut v: Vec<String> = vec![String::from("aa"), String::from("bb"), String::from("cc")];
    for i in &mut v {
        i.push_str("!!");
        println!("{}", i);
    }
}

fn different_element_type_vector() {
    // 不同成员类型的数组
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Hello")),
    ];
    
    for i in row {
        match i {
            SpreadsheetCell::Int(i) => println!("Int: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
    
    // 通过特征对象来实现不同成员类型的数组
    trait Shape {
        fn area(&self) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            self.radius * self.radius * std::f64::consts::PI
        }
    }

    struct Square {
        side: f64,
    }

    impl Shape for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];

    for shape in shapes {
        println!("area = {}", shape.area());
    }
}

pub fn vector() {
    vector_creation();
    vector_iteration();
    different_element_type_vector();
}