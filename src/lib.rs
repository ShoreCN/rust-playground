//! 定义电梯乘客
pub mod passenger {
    /*!
     * 乘客属性
     */
    pub mod property {
        pub fn get_name() {}
        pub fn get_age() {}
        pub fn get_weight() {}
    }

    pub mod behavior {
        pub fn call_elevator() { println!("Call elevator") }
        fn _enter_elevator() {}
        fn _exit_elevator() {}
    }
}

/// # Doc comment
/// function: `doc_line_comment`
/// input: x
/// output: x+1
/// # Examples
/// ```
/// let x = 1;
/// let y = rust_helloworld::doc_line_comment(x);
/// assert_eq!(y, 2);
/// ```
pub fn doc_line_comment(x: i32) -> i32 {
    println!("doc line comment, x={x}");
    x+1
}

/**
    # Doc block comment
    function: `doc_block_comment`
    input: x
    output: x+1
    # Examples
    ```
    let x = 1;
    let y = rust_helloworld::doc_block_comment(x);
    assert_eq!(y, 2);
    ```
 */
pub fn doc_block_comment(x: i32) -> i32 {
    println!("doc block comment, x={x}");
    x+1
}

pub mod doc;
pub use doc::doc_panic;

/** 
 乘客模块测试  
 input: none  
 output: none  
 # Examples
 ```
 rust_helloworld::passenger_test();
 ```
 */
pub fn passenger_test() {
    let _passenger_name = passenger::property::get_name();
    let _passenger_age = crate::passenger::property::get_age();

    passenger::behavior::call_elevator();
    // failed to call enter_elevator, function `enter_elevator` is private
    // passenger::behavior::_enter_elevator();

    doc_line_comment(1);
    doc_block_comment(2);
}


