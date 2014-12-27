#![feature(macro_rules)]

macro_rules! repeat_greeting(
    (
        $(
            $n:expr
        )*
    ) => (
        {
            $(
                println!("Merry Christmas Bill {}!", $n);
            )*
        }
    )
);

/**
  Say Merry Christmas to Bill from 25 to 1.
*/
fn main() {
    repeat_greeting!(25i 24i 23i 22i 21i 20i 19i 18i 17i 16i 15i 14i 13i 12i 11i 10i 9i 8i 7i 6i 5i 4i 3i 2i 1i);
}
