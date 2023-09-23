
fn base_type_array() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    println!("months = {:?}", months);
    println!("second month = {}", months[1]);

    // array with declartion
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // array with same item
    let b = [0; 5];
    println!("array with same item = {:?}", b);

    // out of array access, cause panic
    // let out_of_array_access = b[5]; 

}

fn complex_type_array() {

    let string_array: [String; 3] = std::array::from_fn(|_i| String::from("hello"));
    println!("String_array created by std::array::from_fn = {:?}", string_array);
}

pub fn array_test () {

    base_type_array();

    complex_type_array();
}