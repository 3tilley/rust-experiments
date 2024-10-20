use env_vars::EnvVar;
use std::ops::Add;

use clients;

fn main() {
    println!("{}", env_vars::hello_string());

    for ev in inventory::iter::<EnvVar> {
        println!("{}_{}", ev.env_var_prefix, ev.env_var_name);
    }

    // let d = clients::Doer::new(5);
    let d: usize = 9;
    println!("{}", d.add(7))
}
