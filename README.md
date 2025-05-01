# anti-bankrupt-machine
Custom web-based budgeting manager written in Rust

# Deploy
Run `cargo build --release --target aarch64-unknown-linux-gnu` in repository directory

Move `target/aarch64-unknown-linux-gnu/release/anti-bankrupt-machine` file to `server:~/budgetmanager/backend/bin`

Use scripts to restart server
