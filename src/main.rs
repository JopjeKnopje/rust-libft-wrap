#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use rust_c_bind::{ft_split, ft_strlen};




fn main() {
    let string = "this is a test string hihi";

    let x = unsafe { ft_split(string.as_ptr().cast(), ' ' as i8) };

    dbg!("ft_split {x}", x);
}
