

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

pub fn pattern_test() {
    if_let();
    while_let();
}