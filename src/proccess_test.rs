

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

pub fn process_test() {
    if_process();
}