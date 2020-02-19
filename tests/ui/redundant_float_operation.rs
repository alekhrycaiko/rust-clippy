#![warn(clippy::redundant_float_operation)]
#![allow(
    unused,
    clippy::unnecessary_operation,
)]
fn main() {
    // successes
    print!("{}", 1.132_f32.floor());
    print!("{}", 1.132_f32.ceil());
    print!("{}", 1.132_f32.round());

    print!("{}", 4.432_331_312_f64.floor());
    print!("{}", 4.432_331_312_f64.ceil());
    print!("{}", 4.432_331_312_f64.round());

    // warn

    print!("{}", 1_f32.floor());
    print!("{}", 1_f32.ceil());
    print!("{}", 1_f32.round());

    print!("{}", 4_f64.floor());
    print!("{}", 4_f64.ceil());
    print!("{}", 4_f64.round());
}

