# export RUSTFLAGS = -D warnings
export CARGO_PROFILE_RELEASE_LTO = fat
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS = 1
export DOCKER_BUILDKIT = 1
export CARGO_TARGET_DIR = target

all: run

run ARGS:
	cargo run -- ${ARGS}
	
build-release:
	cargo build -p id-cli --release
