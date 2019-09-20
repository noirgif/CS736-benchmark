# CS736 Assignment 1 -- Benchmark

## Requirement

Rust 2018 version, and nightly is needed. [Rustup](https://rustup.rs) is an easy installer for Rust.

## Running the test

### Local tests

Use `make` to run the tests. Targets are:
- `clock_res` : determine the clock resolution, using `clock_gettime`, `clock_getres`, and `rdtscp`
- `systemcall` : measure system call latency
- `(pipe|tcpip|udp)_(lat|tput)` : measure the latency/throughput of TCP/IP or UDP. Output two columns, the left indicates the packet size, and the right is the time(in ns) or the throughput(in MiB/s).

### Remote tests

Use `cargo` to run the remote tests.

For TCP:
1. Server: Run `cargo run --release --bin us <measure_type> [SERVER_ADDRESS:PORT]` on one machine.
    - Make sure that `[SERVER_ADDRESS:PORT]` is an address the process can bind to. By default it binds to `127.0.0.1:8080`.
    - Use `latency` in the argument to measure latency, use `throughput` to measure throughput.
2. Client: Run `cargo run --release --bin uc <measure_type> [SERVER_ADDRESS:PORT]` on another, to connect to the server.
    - The server address and the measurement type should be the same as step 1.

For UDP:
1. First machine: Run `cargo run --release --bin us <measure_type> <loop_times> [self_address:port] [other_address:port]`.
    - `<measure_type>`: Use `latency` for latency, `throughput` for throughput.
    - `<loop_times>`: how many times the measurement is done. It should be larger(e.g. 100,000) for latency measurement, but small(e.g. 5) for throughput measurement, which takes a century sending 1GiB data each time.
2. Second machine: Run `cargo run --release --bin us <measure_type> <loop_times> [self_address:port] [other_address:port]`
    - The addresses should be in opposite order as in step 1.
