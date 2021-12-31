#! /bin/bash

echo "Running coverage report..."

should_continue() {
    read -p "Coverage report requires Rust nightly, install now? [y/n]: " VALIDATOR

    case $VALIDATOR in
        "y"|"Y"|"yes"|"Yes") echo;;
        "n"|"N"|"no"|"No") echo; echo "Rust nightly not installed, exiting..."; exit 1;;
        *) echo "Invalid input..."; should_continue;;
    esac
}

ACTIVE_TOOLCHAIN=$(rustup show active-toolchain)
if [ ! $(grep -o "nightly" <<< $ACTIVE_TOOLCHAIN) ]; then
    if [ ! $(rustup toolchain list | grep -o "nightly") ]; then
        should_continue
    fi

    echo "Activating rust nightly..."
    rustup default nightly
fi

if [ ! $(rustup component list | grep -o "llvm-tools-preview") ]; then
    echo "Could not find llvm-tools, installing..."
    rustup component add llvm-tools-preview
else
    echo "Found llvm-tools..."
fi

echo "Setting up environment..."
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests"
export RUSTDOCFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests"
export LLVM_PROFILE_FILE="piston_rs-%p-%m.profraw"

echo "Building..."
cargo build

echo "Running tests..."
cargo test

echo "Generating coverage..."
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

echo "Cleaning up..."
rm -rf target/debug/deps/*.gcda
rm -rf target/debug/deps/*.gcno

echo "Reverting to previous default toolchain..."
rustup default $(awk '{print $1}' <<< $ACTIVE_TOOLCHAIN)

echo "Done!"
echo
echo "Test coverage: $(grep -oP "message\":\"\K(\d+%)" target/debug/coverage/coverage.json)"
