use std::ops::Mul;
fn main() {

    // This should suggest expect
    // Stable (1.73.0):     No help             Bad
    // Nightly (1.75.0-n):  No help             Bad
    let abs = 7i32.checked_abs();
    let answer = abs * 5u32; // Case 1

    // This should suggest expect
    // Stable (1.73.0):     Others implement trait `Mul<Rhs>    Bad
    // Nightly (1.75.0-n):  Others implement trait `Mul<Rhs>    Bad
    let abs_2 = 7i32.checked_abs();
    let answer_2 = 5u32 * abs_2; // Case 2

    // This should suggest expect
    // Stable (1.73.0):     Suggested expect    Good :)
    // Nightly (1.75.0-n):  Suggested expect    Good :)
    let abs_3 = 8i32.checked_abs();
    let answer_3 = abs_3.mul(3);

    // This should suggest expect
    // Stable (1.73.0):     Others implement trait `Mul<Rhs>    Bad
    // Nightly (1.75.0-n):  Others implement trait `Mul<Rhs>    Bad
    let abs_4 = 9i32.checked_abs();
    let answer_4 = 9i32.mul(abs_4); // Case 4
}