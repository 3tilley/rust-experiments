# ipc

This is a small proof of concept project for different approaches to Interprocess Communication in Rust

It accompanies a blog post [here](https://3tilley.github.io/posts/simple-ipc-ping-pong/)

## Usage

To demo IPC, run the below, choosing a method from `tcp, udp, shmem, stdout`

`cargo run --release -- -n 1000 --method tcp`

```bash
$ cargo run --release -- -n 1000 --method stdout
    Finished release [optimized] target(s) in 0.10s
     Running `target\release\ipc.exe -n 1000 --method stdout`
IPC method - Stdin/stdout
        1000 cycles completed in 33ms 58us 600ns
        30249.863 per second
        33us 58ns per operation
```

If you want to run the benchmarks, run:

`cargo bench`

```bash
Timer precision: 100 ns
examples                        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ shared_memory                168.7 µs      │ 359.4 µs      │ 186.3 µs      │ 190.9 µs      │ 100     │ 100
│                               5.924 Mitem/s │ 2.781 Mitem/s │ 5.364 Mitem/s │ 5.236 Mitem/s │         │
├─ stdin_stdout                 23.03 ms      │ 30.86 ms      │ 29.53 ms      │ 28.8 ms       │ 100     │ 100
│                               43.4 Kitem/s  │ 32.4 Kitem/s  │ 33.86 Kitem/s │ 34.71 Kitem/s │         │
├─ stdin_stdout_no_preallocate  20.86 ms      │ 33.67 ms      │ 29.42 ms      │ 28.52 ms      │ 100     │ 100
│                               47.93 Kitem/s │ 29.69 Kitem/s │ 33.98 Kitem/s │ 35.06 Kitem/s │         │
├─ tcp_nodelay                  35.28 ms      │ 43.49 ms      │ 41.02 ms      │ 40.47 ms      │ 100     │ 100
│                               28.34 Kitem/s │ 22.99 Kitem/s │ 24.37 Kitem/s │ 24.7 Kitem/s  │         │
├─ tcp_yesdelay                 33.76 ms      │ 43.72 ms      │ 40.9 ms       │ 40.03 ms      │ 100     │ 100
│                               29.62 Kitem/s │ 22.87 Kitem/s │ 24.44 Kitem/s │ 24.97 Kitem/s │         │
╰─ udp                          41.46 ms      │ 46.23 ms      │ 42.98 ms      │ 43.1 ms       │ 100     │ 100
                                24.11 Kitem/s │ 21.62 Kitem/s │ 23.26 Kitem/s │ 23.2 Kitem/s  │         │
```

Note, that if N != 1 in the benches file, the timing data will be for the whole function, not per cycle. However the throughput data is per cycle. This is due to how Divan handles multiple items, which I'm hoping can be improved.

Additionally, because the host process picks out an executable from the targets directory for the consumer, if you make changes to the consumers run `cargo build --release` to make sure they are reflected in the next execution. By
default `cargo run` will only rebuild the `ipc` binrary, which only holds the producer code.

## License

[MIT](https://choosealicense.com/licenses/mit/)
