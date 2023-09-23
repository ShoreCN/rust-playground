

struct Child {
    name: String,
    age: u8,
    mood: String,
}

fn if_process () {
    let child = Child {
        name: String::from("Jack"),
        age: 5,
        mood: String::from("happy"),
    };

    if child.age < 5 {
        println!("{} is a baby", child.name);
    } else if child.age < 10 {
        println!("{} is a child", child.name);
    } else {
        println!("{} is a adult", child.name);
    }

    if child.mood == "happy" {
        println!("{} is happy", child.name);
    } else {
        println!("{} is not happy", child.name);
    }

    let is_prized = if child.mood == "happy" {
        true
    } else {
        false
    }; // mention the semicolon here
    println!("{} is prized = {}", child.name, is_prized);
}

fn for_loop_proccess() {
    // for loop
    for i in 1..=3 {
        println!("Day {}", i);
    }

    let steps = [String::from("first"), String::from("second"), String::from("third")];
    for i in steps {
        println!("step: {}", i);
    }
    // steps is moved to for loop, so it can't be used here
    // println!("steps = {:?}", steps); 

    let mut steps = [String::from("first"), String::from("second")];
    for i in &steps {
        println!("step: {}", i);

    }
    println!("steps = {:?}", steps); // steps is not moved, so it can be used here

    // mutable reference
    for i in &mut steps {
        i.push_str(" step");
        println!("step: {}", i);
    }
    println!("steps = {:?}", steps);

    // enumerate
    for (i, step) in steps.iter().enumerate() {
        println!("element {}: {}", i, step);
    }

    for _ in 0..3 {
        print!("hi!");
    }
}

pub fn process_test() {
    if_process();

    for_loop_proccess();
}