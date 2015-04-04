macro_rules! repeat_greeting {
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
}

/**
  Say Merry Christmas to Bill from 25 to 1.
*/
fn main() {
    repeat_greeting!(25i32 24i32 23i32 22i32 21i32 20i32 19i32 18i32 17i32 16i32 15i32 14i32 13i32 12i32 11i32 10i32 9i32 8i32 7i32 6i32 5i32 4i32 3i32 2i32 1i32);
}
