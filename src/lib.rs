// 定义电梯乘客
pub mod passenger {
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

pub fn passenger_test() {
    let _passenger_name = passenger::property::get_name();
    let _passenger_age = crate::passenger::property::get_age();

    passenger::behavior::call_elevator();
    // failed to call enter_elevator, function `enter_elevator` is private
    // passenger::behavior::_enter_elevator();
}


