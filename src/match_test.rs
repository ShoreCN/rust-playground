#[derive(Debug)]

enum IssueType {
    Bug,
    Task(u32),  // task携带优先级
    Feature,
    Custom(String),
}


fn match_and_if_let() {
    let mut issue_type = IssueType::Bug;
    // 模式匹配, 每个模式返回的类型需要相同
    match issue_type {
        IssueType::Bug => println!("issue type is bug"),
        IssueType::Task(_) => println!("issue type is task"),
        IssueType::Feature => println!("issue type is feature"),
        _ => println!("issue type is unknown"),
    }

    issue_type = IssueType::Custom(String::from("undefined"));
    let issue_val = match issue_type {
        IssueType::Bug => "bug",
        IssueType::Task(_) => "task",
        IssueType::Feature => "feature",
        _ => {
            println!("issue type is unknown, issue type is {:?}", issue_type);
            "unknown"
        },
    };
    println!("issue type is {}", issue_val);

    // 模式绑定
    issue_type = IssueType::Task(34);
    match issue_type {
        IssueType::Task(p) => println!("issue type is task, priority is {}", p),
        other => println!("{:?} is no priority", other),
    }

    // 默认情况可以用_处理, 也可以用变量名处理
    issue_type = IssueType::Feature;
    match issue_type {
        IssueType::Task(p) => println!("issue type is task, priority is {}", p),
        other => println!("{:?} is no priority", other),
    }


    // 如果只有一个模式需要处理, 可以用if let
    let issue_type = IssueType::Custom(String::from("chosen"));
    if let IssueType::Custom(s) = &issue_type {
        println!("issue type is custom, value is {}", s);
    }
    if let IssueType::Task(p) = &issue_type {
        println!("issue type is task, priority is {}", p);
    }
    
}

pub fn match_test() {
    match_and_if_let();
}