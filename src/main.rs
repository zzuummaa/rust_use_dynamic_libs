use crate::my_lib::my_lib_load;

mod my_lib;

fn main() {
    let my_lib = my_lib_load().unwrap();
    unsafe {
        println!("My function returns: {}", my_lib.my_function.into_raw()())
    }
    // my_lib.my_function()
    // println!("my_function: {}", )
}
