#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use rust_c_bind::{ft_strdup, ft_strlen};

fn main() {
    let string = "this is a test string hihi\0";

    unsafe {
        let mut s = ft_strdup(string.as_ptr() as *const i8) as *mut u8;
        let len = ft_strlen(s.cast());

        while s.read() != 0  {
            dbg!("dupped string {}", *s as char);
            s = s.offset(1);
        }
        let sn = String::from_raw_parts(s, len, len);
        println!("sn : {}", sn);
    };

}
