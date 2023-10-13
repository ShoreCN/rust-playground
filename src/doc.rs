
/// # Doc comment
/// function: `doc_line_comment`
/// input: x
/// output: x+1
/// # Examples
/// ```
/// let x = 1;
/// let y = doc_comment(x);
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
    let y = doc_block_comment(x);
    assert_eq!(y, 2);
    ```
 */
pub fn doc_block_comment(x: i32) -> i32 {
    println!("doc block comment, x={x}");
    x+1
}



/// # Doc test
pub fn doc_test() {
    //  line comment

    /*
        block comment 
    */
    println!("doc test");
}