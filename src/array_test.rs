
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

    // 二维数组
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("matrix = {:?}", matrix);
    println!("matrix[1][2] = {}", matrix[1][2]);

    let element = matrix[1][2];
    println!("element = {}", element);

}

fn complex_type_array() {

    let string_array: [String; 3] = std::array::from_fn(|_i| String::from("hello"));
    println!("String_array created by std::array::from_fn = {:?}", string_array);

    // 二维数组
    let string_matrix: [[String; 3]; 2] = std::array::from_fn(|_i| std::array::from_fn(|_j| String::from("hello")));
    println!("String_matrix created by std::array::from_fn = {:?}", string_matrix);
    // pick random element
    println!("String_matrix[1][2] = {}", string_matrix[1][2]);

    let element = string_matrix[1][2].clone();
    println!("element = {}", element);

    let ref_element = &string_matrix[1][2];
    println!("ref_element = {}", ref_element);
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("array {:?} slice -> {:?}", a, slice);
}

pub fn array_test () {

    base_type_array();

    complex_type_array();

    array_slice();
}