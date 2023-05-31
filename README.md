# segway-pid

This project is an exercise in implementing a simple Proportional-Integral-Derivative (PID) controller using the Rust programming language. The goal of this exercise is to explore the Rust language and its features while learning about PID controllers and their applications in various control systems. The code is not very organized. 

## Getting Started

To build this project into Web Assembly:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
cargo install basic-http-server
basic-http-server static
```
![Screenshot 2023-05-31 at 12 48 53 AM](https://github.com/alaney2/segway-pid/assets/70783523/dfc2aab3-8f98-4a62-a7b9-ddb9df70f465)
