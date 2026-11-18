#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


include!("bindings.rs");

fn main() {
    unsafe {

    let x = hello();
    println!("x {}",x);
    }
    println!("Hello, world!");
}
