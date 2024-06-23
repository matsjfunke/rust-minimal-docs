# Installation / Update
```bash
# install with rustup:
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
# update_
rustup update
```

# Cargo: system and package manager
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory
- create a project using cargo new.
- build a project using cargo build.
- build and run a project in one step using cargo run.
- build a project without producing a binary to check for errors using cargo check.
- update dependencies using cargo update
