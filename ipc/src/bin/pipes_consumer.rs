use std::io::{Read, stdin, stdout, Write};

fn main() {
    let mut input = String::with_capacity(5);
    let mut arr = [0u8,0,0,0,0];
    // let input_len = stdin().read_line(&mut input).unwrap();
    loop {
        let read_result = stdin().read_exact(&mut arr);
        if read_result.is_ok() {
            // eprintln!("Got input '{:?}' (length {})", arr, arr.len() );
            let output = match &arr {
                b"ping\n" => b"pong\n",
                b"pong\n" => b"ping\n",
                _ => b"Error"
            };
            stdout().write(output).unwrap();
        }
    }
}