extern crate libc;

extern {
    fn my_function() -> i32;
}

fn main() {
    unsafe {
        println!("My function returns: {}", my_function())
    }
    // my_lib.my_function()
    // println!("my_function: {}", )
}
