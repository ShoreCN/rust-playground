use std::vec;



struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

// 为Cacher实现方法
impl <T> Cacher<T>
    where T: Fn(u32) -> u32
{
    // 为Cacher实现new方法
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // 为Cacher实现value方法
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// 实现FnMut的闭包
fn fn_mut_closure() {
    let mut num = 5;
    let mut add_num = |x: u32| num += x;
    add_num(5);
    println!("num = {}", num);

    // 无法通过编译, 因为没有声明mut, 闭包默认是不可变的
    // let add_num2 = |x: u32| num += x;
    // add_num2(5);

    let c = |x| x;
    let c5 = c(5);
    let c7 = c(7);
    println!("c(5) = {c5}, c(7) = {c7}");

    let p1: u32 = 10;
    let fn_once = move |x: u32| x;
    let fn_once5 = fn_once(p1);
    // 可以通过编译, 因为闭包已经move了p1的所有权, 但是p1的类型是u32, 实现了Copy trait, 所以可以继续使用
    let fn_once7 = fn_once(p1);
    println!("fn_once(5) = {fn_once5}, fn_once(7) = {fn_once7}");
    let p2 = vec![1, 2, 3];
    let fn_once2 = move |x| x;
    let fn_once2_1 = fn_once2(p2);
    // 无法通过编译, 因为闭包已经move了p2的所有权, 所以无法继续使用
    // let fn_once2_2 = fn_once2(p2);
    println!("fn_once2_1 = {:?}", fn_once2_1);
    // println!("p2 = {:?}", p2);

}

fn cacher_test() {
    let mut cacher = Cacher::new(|a| a);
    let v1 = cacher.value(1);
    let v2 = cacher.value(2);
    println!("v1 = {}, v2 = {}", v1, v2);
}

fn use_value_in_closure() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("equal_to_x(y) = {}", equal_to_x(y));
}

pub fn closure() {
    cacher_test();
    use_value_in_closure();
    fn_mut_closure();
}