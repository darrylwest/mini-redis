
project := "mini-redis"

alias t := test
alias ta := test-all
alias b := build
alias rel := release

# run the standard tests
test:
    clear
    cargo test

# run the standard tests + clippy and fmt
test-all:
    clear
    cargo test && cargo fmt && cargo clippy

# build the debug target
build:
    clear
    cargo build

# build the docs
docs:
    cargo doc --no-deps --open

# run the debug app
run-server:
    clear && cargo run --bin db-server

# build the release
release:
    clear
    cargo build --release --bin

# watch the current folders and run tests when a file is changed
watch:
    watchexec -d 500 -c -e rs cargo test && cargo fmt && cargo clippy

# cover - runs code test coverage report and writes to coverage folder
cover:
  cargo tarpaulin --out html --output-dir coverage

update-piedmont:
    ssh dpw@piedmont 'cd ~/raincity/rust-projects/{{ project }} && git pull'

update-tiburon:
    ssh dpw@tiburon 'cd ~/raincity/rust-projects/{{ project }} && git pull'

update-remotes:
    just update-piedmont
    just update-tiburon
