use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
fn main() {
    let x = 0;
    println!("Iterating from 0..={x}");
    for i in 0..=x {
        println!("{i}")
    }
    let y = 1;
    println!("Iterating from 0..={y}");
    for j in 0..=y {
        println!("{j}")
    }

    let a = 0;
    println!("Iterating from {a}..=1");
    for k in a..=1 {
        println!("{k}")
    }
    let b = 1;
    println!("Iterating from {b}..=0");
    for h in b..=0 {
        println!("{h}")
    }

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }
}
