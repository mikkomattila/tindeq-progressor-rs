# tindeq-progressor-rs

A simple command-line utility to connect to a Tindeq Progressor dynamometer via Bluetooth.

TODO: Do something useful after the fact.

## Prerequisites

- You need to have a Tindeq Progressor device and it must be turned on. One can be bought from [Tindeq](https://tindeq.com/).
- You need to have both `Rust` and `Cargo` installed, see: [rust-lang.org](https://www.rust-lang.org/tools/install).
  - Additionally you may need to install system dependencies required by the [`btleplug`](https://crates.io/crates/btleplug) crate, depending on your operating system.

## Usage

Once you have the prerequisites installed, you can either run or build the application using Cargo. See the examples below.

**Run:**

```sh
cargo run --release
```

**Build:**

1. Build with release configuration:

    ```sh
    cargo build --release
    ```

2. Produced binary can be found from `src/target/release` and ran with the following command from project root:

    ```sh
    ./target/release/tindeq-progressor-rs
    ```
