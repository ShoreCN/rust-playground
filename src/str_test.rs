pub fn slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello = {}, world = {}", hello, world);

    let hello = &s[..5];
    let world = &s[6..];
    println!("hello = {}, world = {}", hello, world);

    let hello = &s[..];
    println!("hello = {}", hello);

    let hello = &s[0..=4];
    let world = &s[6..=10];
    println!("hello = {}, world = {}", hello, world);

}

// fn main() {
//     slice();
// }