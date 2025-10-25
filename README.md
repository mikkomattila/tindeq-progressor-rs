# tindeq-progressor-rs

A simple command-line utility to connect to a Tindeq Progressor dynamometer via Bluetooth.

TODO: Do something useful after the fact.

## Prerequisites

Before you can build and run this project, you may need to install system dependencies required by the `btleplug` crate, depending on your operating system.

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
