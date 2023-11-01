
#![feature(stmt_expr_attributes)]
// stmt_expr_attributes
// Tracking issue: https://github.com/rust-lang/rust/issues/15701

//use literal_matches;
fn main() {
    // println!("Hello, world!");

    // Note that this is a statement. Maybe because it starts with let?
    #[literal_matches::return_as_is]
    let ans = {
        match true {
            true => 1,
            false => 0
        }
    };

    // This is an expression
    #[literal_matches::return_as_is]
    match ans {
        1 => println!("It was true!"),
        2 => println!("It was not!"),
    }

    ()
}