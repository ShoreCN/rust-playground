

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
}