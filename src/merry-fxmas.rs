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
)

fn main() {
    #![doc = "Say Merry Christmas to Bill from 25 to 1."]
    repeat_greeting!(25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1);
}