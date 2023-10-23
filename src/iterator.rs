use crate::elevator::FloorType;


fn array_iter(){
    let arr = [1, 2, 3, 4, 5];
    for _i in arr {
    }
    for _i in &arr {
    }
    for _i in arr.iter() {
    }
    for _i in arr.into_iter() {
    }

    let mut v = vec![1, 2, 3, 4, 5];
    // 使用for循环遍历vector之后, vector的所有权会被转移, 后续无法再次使用
    // for _i in v {
    // }
    for _i in &v {
    }
    for _i in v.iter() {
    }
    // 可变借用, 可以修改vector的值
    for _i in v.iter_mut() {
    }

    // 使用into_iter()方法, vector的所有权会被转移, 后续无法再次使用
    for _i in v.into_iter() {
    }

}

pub fn iterator(){
    array_iter();
}