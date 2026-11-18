#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use rust_c_bind::ft_strchr;

fn main() {
    let string = "this is a test string hihi\0";

    let x = unsafe { *ft_strchr(string.as_ptr().cast(), 'g' as i32) };

    println!("found {} in string: {}", x.cast_unsigned() as char, string);
}
