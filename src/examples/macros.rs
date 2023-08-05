use crate::public_add;

// need to be before main to be used in main
macro_rules! add {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $a + $b
    };
    ($a:expr, $($b:expr),+) => {
        $a + add!($($b),+)
    };
}

pub fn main()
{
    println!("Result1: {}", add!(1, 2));
    println!("Result2: {}", add!(1, 2, 3, 4));

    println!("Result1: {}", crate::public_add!(1, 2));
    println!("Result2: {}", crate::public_add!(1, 2, 3, 4));
}

#[macro_export]
macro_rules! public_add {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $a + $b
    };
    ($a:expr, $($b:expr),+) => {
        $a + public_add!($($b),+)
    };
}
