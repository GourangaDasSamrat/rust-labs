#[allow(clippy::pedantic)]
pub const fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

#[allow(clippy::arithmetic_side_effects)]
const fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}
