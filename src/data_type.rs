use std::fmt::{self, Display};

// new type
// 作用1: 提升可读性
// 作用2: 为外部类型实现外部trait
// 通过newtype可以为外部类型实现自己的trait, 避免因为孤儿规则而无法为外部类型实现外部trait
// 样例可以参考src/output.rs: impl fmt::Display for Wrapper
// 作用3: 隐藏内部实现

struct Floor(u8);

impl Display for Floor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "第{}层", self.0)
    }
}

// type alias
type FloorNum = u8;
type ShortBox = Box<dyn Display + Send + 'static>;

// 大幅减少重复代码
fn sample(i: ShortBox) -> ShortBox {
    let other: ShortBox = Box::new(i);
    other
}

pub fn data_type() {
    let floor = Floor(1);
    println!("floor = {}", floor);

    let mut floor_num: FloorNum = 1;
    floor_num += 1;
    println!("floor_num = {}", floor_num);

    let i: ShortBox = Box::new(1);
    let _j = sample(i);
}