version := "0.0.1"
output  := "./target/debug/id-cli"

alias r := run
# alias t := test
# alias c := check
alias A := add
alias b := build

default: run

run:
    cargo run -p id-cli

build +PACKAGE="id-cli":
    cargo build -p {{PACKAGE}}

publish:
  #!/usr/bin/env zsh
  set -euxo pipefail
  rm -rf ./target/release

add PACKAGE='id-cli' CRATE='' +FEATURES='':
    @echo "\x1b[32;1mAdding {{CRATE}}\x1b[35;1m {{FEATURES}}\x1b[0m to\x1b[35;1m {{PACKAGE}}..."
    cargo add -p {{PACKAGE}} {{CRATE}} --features {{FEATURES}}

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
    
