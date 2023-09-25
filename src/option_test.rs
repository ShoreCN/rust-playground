
// Option<T> is an enum with two variants:
// enum Option<T> {
//     None,
//     Some(T),
// }

fn plus_one(number: Option<i32>) -> Option<i32> {
    match number {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn option_test() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number = {:?}", some_number);
    println!("some_string = {:?}", some_string);
    println!("absent_number = {:?}", absent_number);

    println!("plus_one(some_number) = {:?}", plus_one(some_number));
    // println!("plus_one(some_string) = {:?}", plus_one(some_string)); // error, mismatched types
    println!("plus_one(absent_number) = {:?}", plus_one(absent_number));
}