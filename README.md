# anti-bankrupt-machine
Custom web-based budgeting manager written in Rust

# Deploy
Run `cargo build --release` in repository directory

Zip `target/release/` file

On server, delete everything in `budgetmanager` folder except for `Rocket.toml`

Unzip `release.zip` in `budgetmanager`

Use scripts to restart server
