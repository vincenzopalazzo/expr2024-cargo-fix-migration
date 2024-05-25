#![feature(expr_fragment_specifier_2024)]

macro_rules! m {
    ($e:expr_2021) => {
        $e
    };
}

fn main() {
    m!(());
}
