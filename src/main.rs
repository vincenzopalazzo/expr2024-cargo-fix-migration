#![feature(expr_fragment_specifier_2024)]

macro_rules! m {
    ($e:expr) => {
        $e
    };
}

fn main() {
    m!(());
}
