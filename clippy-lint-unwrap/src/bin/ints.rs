use std::convert::TryInto;
use std::ops::Add;

fn main() {

    // This should suggest expect
    // Stable (1.73.0):     No help             Bad
    // Nightly (1.75.0-n):  No help             Bad
    // This PR:             Doesn't             Bad
    let my_int = "3".try_into().ok();
    let answer = my_int + 4; // Case 1

    // This should suggest expect
    // Stable (1.73.0):     No help             Bad
    // Nightly (1.75.0-n):  Suggest expect      Good :)
    // This PR:             Suggest expect      Good :)
    let suggest_int: u32 = "2".try_into(); // Case 2

    // This should suggest expect
    // Stable (1.73.0):     Remove method call  Bad
    // Nightly (1.75.0-n):  Remove method call  Bad
    // This PR:             Expect              Good :)
    let abs: i32 = 3i32.checked_abs(); // Case 3

    // This should suggest removing method call
    // Stable (1.73.0):     Remove method call  Good :)
    // Nightly (1.75.0-n):  Remove method call  Good :)
    // This PR:             Remove method call  Good :)
    let suggest_int: u32 = 2.to_string(); // Case 4

    // This should suggest removing method call
    // Stable (1.73.0):     Remove method call  Good :)
    // Nightly (1.75.0-n):  Remove method call  Good :)
    // This PR:             Remove method call  Good :)
    let suggest_int: u32 = 2.to_string().try_into(); // Case 5


    // This should suggest expect
    // Stable (1.73.0):     No help             Bad
    // Nightly (1.75.0-n):  No help             Bad
    // This PR:             No help             Bad
    let abs_2 = 7i32.checked_abs();
    let answer_2 = abs_2 + 5u32; // Case 6

    // This should suggest expect
    // Stable (1.73.0):     No help             Bad
    // Nightly (1.75.0-n):  No help             Bad
    // This PR:             No help             Bad
    let abs_7 = 7i32.checked_abs();
    let answer_7 = 5u32 + abs_7; // Case 6

    let abs_8 = 8i32.checked_abs();
    let answer_8 = abs_8.add(9);

    let abs_9 = 9i32.checked_abs();
    let answer_9 = 9.add(abs_9);

}
