# anti-bankrupt-machine
Custom web-based budgeting manager written in Rust

# Deploy
Run `cargo build --release` in repository directory

Move `target/release/anti-bankrupt-machine` file to `server:~/budgetmanager`

Run `zip -r webapp.zip static/ templates/` and transfer to `server:~/budgetmanager`

Delete `server:~/budgetmanager/static/`, `server:~/budgetmanager/templates/`

Unzip `webapp.zip` into `server:~/budgetmanager` and delete zip

Use scripts to restart server
