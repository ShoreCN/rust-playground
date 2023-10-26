

fn type_transfer() {
    let x: i32 = 1000000000;
    let y: i64 = x.into();
    println!("max of i32 is {}, max of i64 is {}", i32::MAX, i64::MAX);
    println!("x = {}, y = {}", x, y);

    // error: 这里无法通过into()转换, 因为i64的范围超过了i32, 但是通过as转换可以, 因为这里只是简单的截断
    // let x: i64 = 1000000000000000000;
    // let y: i32 = x.into();
    // println!("x = {}, y = {}", x, y);

    let x: i32 = 1000000000;
    let y: i64 = x as i64;
    println!("x = {}, y = {}", x, y);

    // 虽然可以通过as转换, 但是这里会有溢出的风险, 所以产生的结果是错误的
    let x: i64 = 1000000000000000000;
    let y: i32 = x as i32;
    println!("x = {}, y = {}", x, y);

    // try_into, 通过try_into转换, 如果转换失败, 则会返回错误
    let try_ok: Result<i32, _> = (1000000000 as i64).try_into();
    let try_err: Result<i32, _> = (1000000000000000000 as i64).try_into();
    for i in [try_ok, try_err].iter() {
        match i {
            Ok(x) => println!("try_into ok, x = {}", x),
            Err(e) => println!("try_into err, e = {}", e),
        }
    }
    
}

pub fn transfer() {
    type_transfer();
}