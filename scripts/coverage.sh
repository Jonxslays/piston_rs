#!/bin/bash

# Make sure grcov is installed
which grcov &> /dev/null
if [ $? != 0 ]; then
    echo "grcov is not installed, please install it with \`cargo install grcov\`."
    exit 1
fi

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
export RUSTFLAGS="-Cinstrument-coverage"
export RUSTDOCFLAGS="-Cinstrument-coverage -Zunstable-options --persist-doctests target/debug/doctestbins"
export LLVM_PROFILE_FILE="piston_rs-%p-%m.profraw"

echo "Running tests..."
cargo test

echo "Generating coverage report..."
grcov . -s . -t html -o ./target/debug/coverage/ \
    --binary-path ./target/debug/ \
    --ignore-not-existing \
    --branch \
    --excl-br-line "#\[derive\(" \
    --excl-line "#\[derive\("

echo
echo "Cleaning up..."
rm -f ./*.profraw
rm -rf ./target/debug/doctestbins

echo
echo "Reverting to previous default toolchain..."
rustup default $(awk '{print $1}' <<< $ACTIVE_TOOLCHAIN)

echo "Done!"
echo
echo "Test coverage: $(grep -oP "message\":\"\K(\d+%)" target/debug/coverage/coverage.json)"
