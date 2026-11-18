#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use rust_c_bind::ft_strlen;




fn main() {
    let string = "this is a test string hihi";

    let len = unsafe { ft_strlen(string.as_ptr().cast()) };

    dbg!("ft_split {x}", len);
}
