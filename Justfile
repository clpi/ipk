version := "0.0.1"
output  := "./target/debug/id-cli"


default: run

run:
    cargo run -p id-cli

add CRATE:
    @echo "\x1b[32;1mAdding {{CRATE}}..."
    cargo add -p id-cli {{CRATE}}

addp PACKAGE CRATE:
    cargo add -p {{PACKAGE}} {{CRATE}}

addpf PACKAGE CRATE FEATURES:
    cargo add -p {{PACKAGE}} {{CRATE}} --features {{FEATURES}}

add-cli CRATE FEATURES:
    cargo add -p {{CRATE}} --features {{FEATURES}}

test-pkg PACKAGE:
    @echo "\x1b[33;1mRunning {{PACKAGE}} tests... \x1b[0m"
    cargo test

build-pkg PACKAGE:
    @echo "\x1b[33;1mBuilding {{PACKAGE}}... \x1b[0m"
    cargo build

run-pkg PACKAGE:
    @echo "\x1b[32;1mRunning {{PACKAGE}}...\x1b[0m"
    cargo run -p id-cli

bench-pkg PACKAGE:
    @echo "\x1b[32;1mBenchmarking {{PACKAGE}}...\x1b[0m"
    cargo bench -p id-cli
    
