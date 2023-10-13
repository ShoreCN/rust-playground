
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



/**
 * # 文档注释中的panic处理
 * function: `doc_panic`  
 * input: `x<i32>`  
 * output: `x+1`  
 * # Examples
 * ```
 * let x = 1;
 * let y = rust_helloworld::doc_panic(x);
 * assert_eq!(y, 2);
 * ```
 * # Panics
 * ```should_panic
 * let x = -1;
 * let y = rust_helloworld::doc_panic(x);
 * assert_eq!(y, 2);
 * ```
 * 
 */
pub fn doc_panic(x: i32) -> i32 {
    if x < 0 {
        panic!("x < 0");
    }
    x+1
}


/// # 通过目录功能测试文档注释中的跳转能力
/// function: `doc_dir`
/// [`doc_line_comment`]
/// [`doc_block_comment`]
/// [`doc_panic`]
pub fn doc_dir() {
}


/// # Doc test
pub fn doc_test() {
    //  line comment
    doc_line_comment(1);

    /*
        block comment 
    */
    println!("doc test");
}