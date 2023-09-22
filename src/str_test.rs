pub fn slice() {
    println!("[slice]");
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

    let a = [1, 2, 3, 4, 5];
    let a_slice = &a[1..3];
    println!("a_slice = {:?}", a_slice);
}

pub fn str_op() {
    println!("[str_op]");

    let mut s = String::from("hello");  // push_str会改变变量的内容, 所以必须是mut
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = String::from(", world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let s = String::from("hello");
    let h = &s[0..1];
    println!("h = {}", h);

    let mut s = String::from("number one is 1");    // insert会改变变量的内容, 所以必须是mut
    s.insert(s.len(), '!');
    println!("s insert char, s = {}", s);
    s.insert_str(0, "Mike: ");
    println!("s insert str, s = {}", s);

    let s = String::from("This sentence is a unique sentence.");
    let s2 = s.replace("unique", "copy");   // replace会返回一个新的字符串, 所以不需要是mut
    println!("str replaced, s2 = {}", s2);
    let s3 = s.replacen("sentence", "notice", 1);
    println!("str replaced once(by replacen), s3 = {}", s3);
    let mut s = String::from("This sentence is a unique sentence.");
    s.replace_range(0..4, "That");  // replace_range会改变变量的内容, 所以必须是mut
    println!("str replaced range, s = {}", s);

    let len = String::from("Hola").len();
    println!("len = {}", len);

    let len = String::from("Здравствуйте").chars().count();
    println!("len = {}", len);
}

// fn main() {
//     slice();
// }