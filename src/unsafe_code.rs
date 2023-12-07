

fn raw_pointer() {
    let mut x = 1;
    let const_raw_ptr = &x as *const i32;
    let mut_raw_ptr = &mut x as *mut i32;

    unsafe {
        println!("const_raw_ptr = {}", *const_raw_ptr);
        *mut_raw_ptr = 2;
        println!("mut_raw_ptr = {}", *mut_raw_ptr);
        println!("const_raw_ptr = {}", *const_raw_ptr);
    }
}


pub fn unsafe_code() {
    raw_pointer();
}